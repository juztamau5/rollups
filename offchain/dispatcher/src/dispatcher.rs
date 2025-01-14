// Copyright Cartesi Pte. Ltd.
//
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use rollups_events::{Address, DAppMetadata};
use state_client_lib::StateServer;
use state_fold_types::{Block, BlockStreamItem};
use tokio_stream::StreamExt;
use tracing::{error, info, instrument, trace, warn};
use types::foldables::authority::rollups::{RollupsInitialState, RollupsState};

use crate::{
    config::DispatcherConfig,
    drivers::{blockchain::BlockchainDriver, machine::MachineDriver, Context},
    error::{BrokerSnafu, DispatcherError, SenderSnafu, StateServerSnafu},
    machine::{rollups_broker::BrokerFacade, BrokerReceive, BrokerSend},
    sender::ClaimSender,
    setup::{create_block_subscription, create_context, create_state_server},
};

use snafu::{whatever, ResultExt};

#[instrument(level = "trace", skip_all)]
pub async fn start(config: DispatcherConfig) -> Result<(), DispatcherError> {
    info!("Setting up dispatcher with config: {:?}", config);

    trace!("Creating transaction manager");
    let mut claim_sender =
        ClaimSender::new(&config).await.context(SenderSnafu)?;

    trace!("Creating state-server connection");
    let state_server = create_state_server(&config.sc_config).await?;

    trace!("Starting block subscription with confirmations");
    let mut block_subscription = create_block_subscription(
        &state_server,
        config.sc_config.default_confirmations,
    )
    .await?;

    trace!("Creating broker connection");
    let broker = BrokerFacade::new(
        config.broker_config.clone(),
        DAppMetadata {
            chain_id: config.tx_config.chain_id,
            dapp_address: Address::new(
                config.dapp_deployment.dapp_address.into(),
            ),
        },
    )
    .await
    .context(BrokerSnafu)?;

    trace!("Creating context");
    let mut context = create_context(&config, &state_server, &broker).await?;

    trace!("Creating machine driver and blockchain driver");
    let mut machine_driver =
        MachineDriver::new(config.dapp_deployment.dapp_address);
    let mut blockchain_driver =
        BlockchainDriver::new(config.dapp_deployment.dapp_address);

    let initial_state = RollupsInitialState {
        history_address: config.rollups_deployment.history_address,
        input_box_address: config.rollups_deployment.input_box_address,
    };

    trace!("Starting dispatcher...");
    loop {
        match block_subscription.next().await {
            Some(Ok(BlockStreamItem::NewBlock(b))) => {
                // Normal operation, react on newest block.
                trace!(
                    "Received block number {} and hash {:?}, parent: {:?}",
                    b.number,
                    b.hash,
                    b.parent_hash
                );
                claim_sender = process_block(
                    &b,
                    &state_server,
                    &initial_state,
                    &mut context,
                    &mut machine_driver,
                    &mut blockchain_driver,
                    &broker,
                    claim_sender,
                )
                .await?
            }

            Some(Ok(BlockStreamItem::Reorg(bs))) => {
                error!(
                    "Deep blockchain reorg of {} blocks; new latest has number {:?}, hash {:?}, and parent {:?}",
                    bs.len(),
                    bs.last().map(|b| b.number),
                    bs.last().map(|b| b.hash),
                    bs.last().map(|b| b.parent_hash)
                );
                error!("Bailing...");
                whatever!("deep blockchain reorg");
            }

            Some(Err(e)) => {
                warn!(
                    "Subscription returned error `{}`; waiting for next block...",
                    e
                );
            }

            None => {
                whatever!("subscription closed");
            }
        }
    }
}

#[instrument(level = "trace", skip_all)]
async fn process_block(
    block: &Block,

    state_server: &impl StateServer<
        InitialState = RollupsInitialState,
        State = RollupsState,
    >,
    initial_state: &RollupsInitialState,

    context: &mut Context,
    machine_driver: &mut MachineDriver,
    blockchain_driver: &mut BlockchainDriver,

    broker: &(impl BrokerSend + BrokerReceive),

    claim_sender: ClaimSender,
) -> Result<ClaimSender, DispatcherError> {
    trace!("Querying rollup state");
    let state = state_server
        .query_state(initial_state, block.hash)
        .await
        .context(StateServerSnafu)?;

    // Drive machine
    trace!("Reacting to state with `machine_driver`");
    machine_driver
        .react(context, &state.block, &state.state.input_box, broker)
        .await
        .context(BrokerSnafu)?;

    // Drive blockchain
    trace!("Reacting to state with `blockchain_driver`");
    blockchain_driver
        .react(&state.state.history, broker, claim_sender)
        .await
}
