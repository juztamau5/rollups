// Copyright Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

/// @title IPFS Portal Test
pragma solidity ^0.8.8;

import {Test} from "forge-std/Test.sol";
import {IIPFSPortal} from "contracts/portals/{IIPFSPortal.sol";
import {IPFSPortal} from "contracts/portals/{IIPFSPortal.sol";
import {IInputBox} from "contracts/inputs/IInputBox.sol";
import {InputBox} from "contracts/inputs/InputBox.sol";

contract IPFSPortalTest is Test {
    IInputBox inputBox;
    IIPFSPortal portal;

    event InputAdded(
        address indexed dapp,
        uint256 indexed inboxInputIndex,
        address sender,
        bytes input
    );

    function setUp() public {
        inputBox = new InputBox();
        portal = new IPFSPortal(inputBox);
    }

    function testGetInputBox() public {
        assertEq(address(portal.getInputBox()), address(inputBox));
    }

    function testIPFSDeposit(bytes calldata _ipfsData) public {
        // Check the DApp's input box before
        assertEq(inputBox.getNumberOfInputs(_dapp), 0);

        // Construct the IPFS deposit input
        bytes memory input = abi.encodePacked(_ipfsData);

        // Expect InputAdded to be emitted with the right arguments
        vm.expectEmit(true, true, false, true, address(inputBox));
        emit InputAdded(dapp, 0, address(portal), input);

        // Check the DApp's input box after
        assertEq(inputBox.getNumberOfInputs(_dapp), 1);
    }
}
