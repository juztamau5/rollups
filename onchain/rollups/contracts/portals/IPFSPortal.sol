// Copyright Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

pragma solidity ^0.8.8;

import {IIPFSPortal} from "./IIPFSPortal.sol";
import {Portal} from "./Portal.sol";
import {IInputBox} from "../inputs/IInputBox.sol";
import {InputEncoding} from "../common/InputEncoding.sol";

/// @title IPFS Portal
///
/// @notice This contract allows anyone to perform transfers of
/// IPFS data to a DApp while informing the off-chain machine.
contract IPFSPortal is Portal, IIPFSPortal {
    /// @notice Constructs the portal.
    /// @param _inputBox The input box used by the portal
    constructor(IInputBox _inputBox) Portal(_inputBox) {}

    function transferIPFS(
        address _dapp,
        bytes calldata _ipfsData,
        bytes calldata _execLayerData
    ) external override {
        bytes memory input = InputEncoding.encodeIPFSTransfer(
            msg.sender,
            _ipfsData,
            _execLayerData
        );

        inputBox.addInput(_dapp, input);
    }
}
