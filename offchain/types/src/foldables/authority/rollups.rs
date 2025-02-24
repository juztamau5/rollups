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

use crate::{
    foldables::{
        claims::{History, HistoryInitialState},
        input_box::{InputBox, InputBoxInitialState},
    },
    FoldableError, UserData,
};

use state_fold::{
    FoldMiddleware, Foldable, StateFoldEnvironment, SyncMiddleware,
};
use state_fold_types::{
    ethers::{providers::Middleware, types::Address},
    Block, QueryBlock,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct RollupsInitialState {
    pub history_address: Address,
    pub input_box_address: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RollupsState {
    pub input_box_initial_state: Arc<InputBoxInitialState>,
    pub input_box: Arc<InputBox>,

    pub history_initial_state: Arc<HistoryInitialState>,
    pub history: Arc<History>,
}

#[async_trait]
impl Foldable for RollupsState {
    type InitialState = Arc<RollupsInitialState>;
    type Error = FoldableError;
    type UserData = Mutex<UserData>;

    async fn sync<M: Middleware + 'static>(
        initial_state: &Self::InitialState,
        block: &Block,
        env: &StateFoldEnvironment<M, Self::UserData>,
        _access: Arc<SyncMiddleware<M>>,
    ) -> Result<Self, Self::Error> {
        let (input_box_initial_state, history_initial_state) = {
            let mut user_data = env
                .user_data()
                .lock()
                .expect("Mutex should never be poisoned");

            let i = {
                let input_box_address =
                    user_data.get(initial_state.input_box_address);
                Arc::new(InputBoxInitialState { input_box_address })
            };

            let h = {
                let history_address =
                    user_data.get(initial_state.history_address);
                Arc::new(HistoryInitialState { history_address })
            };

            (i, h)
        };

        fetch_sub_foldables(
            env,
            block,
            input_box_initial_state,
            history_initial_state,
        )
        .await
    }

    async fn fold<M: Middleware + 'static>(
        previous_state: &Self,
        block: &Block, // TODO: when new version of state-fold gets released, change this to Arc
        // and save on cloning.
        env: &StateFoldEnvironment<M, Self::UserData>,
        _access: Arc<FoldMiddleware<M>>,
    ) -> Result<Self, Self::Error> {
        fetch_sub_foldables(
            env,
            block,
            previous_state.input_box_initial_state.clone(),
            previous_state.history_initial_state.clone(),
        )
        .await
    }
}

async fn fetch_sub_foldables<M: Middleware + 'static>(
    env: &StateFoldEnvironment<M, <RollupsState as Foldable>::UserData>,
    block: &Block,
    input_box_initial_state: Arc<InputBoxInitialState>,
    history_initial_state: Arc<HistoryInitialState>,
) -> Result<RollupsState, <RollupsState as Foldable>::Error> {
    // TODO: Change state-fold sync/fold to receive Arc<Block>
    let block = QueryBlock::Block(Arc::new(block.clone()));

    let input_box = env
        .get_state_for_block::<InputBox>(
            &input_box_initial_state,
            block.clone(),
        )
        .await?
        .state;

    let history = env
        .get_state_for_block::<History>(&history_initial_state, block)
        .await?
        .state;

    Ok(RollupsState {
        input_box,
        input_box_initial_state,
        history,
        history_initial_state,
    })
}
