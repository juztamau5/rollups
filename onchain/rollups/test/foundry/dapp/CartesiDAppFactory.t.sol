// Copyright 2023 Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

/// @title Cartesi DApp Factory Test
pragma solidity ^0.8.8;

import {TestBase} from "../util/TestBase.sol";
import {SimpleConsensus} from "../util/SimpleConsensus.sol";
import {CartesiDAppFactory} from "contracts/dapp/CartesiDAppFactory.sol";
import {CartesiDApp} from "contracts/dapp/CartesiDApp.sol";
import {IConsensus} from "contracts/consensus/IConsensus.sol";
import {Vm} from "forge-std/Vm.sol";

contract CartesiDAppFactoryTest is TestBase {
    CartesiDAppFactory factory;
    IConsensus consensus;

    function setUp() public {
        factory = new CartesiDAppFactory();
        consensus = new SimpleConsensus();
    }

    event ApplicationCreated(
        IConsensus indexed consensus,
        address dappOwner,
        bytes32 templateHash,
        CartesiDApp application
    );

    function testNewApplication(
        address _dappOwner,
        bytes32 _templateHash
    ) public {
        vm.assume(_dappOwner != address(0));

        CartesiDApp newDapp = factory.newApplication(
            consensus,
            _dappOwner,
            _templateHash
        );

        assertEq(address(newDapp.getConsensus()), address(consensus));
        assertEq(newDapp.owner(), _dappOwner);
        assertEq(newDapp.getTemplateHash(), _templateHash);
    }

    function testApplicationCreatedEvent(
        address _dappOwner,
        bytes32 _templateHash
    ) public {
        vm.assume(_dappOwner != address(0));

        // Start the recorder
        vm.recordLogs();

        // perform call and emit event
        CartesiDApp newDapp = factory.newApplication(
            consensus,
            _dappOwner,
            _templateHash
        );

        // get recorder logs
        Vm.Log[] memory entries = vm.getRecordedLogs();

        // there is at least one entry
        assertGt(entries.length, 0);

        // get last log entry
        Vm.Log memory entry = entries[entries.length - 1];

        // there are 2 topics
        assertEq(entry.topics.length, 2);

        // topics[0] is the event signature
        assertEq(
            entry.topics[0],
            keccak256("ApplicationCreated(address,address,bytes32,address)")
        );

        // topics[1] is the IConsensus parameter
        // restrictions on explicit type convertions:
        // "The conversion is only allowed when there is at most one change in sign, width or type-category"
        // ref: https://docs.soliditylang.org/en/latest/080-breaking-changes.html#new-restrictions
        assertEq(
            entry.topics[1],
            bytes32(uint256(uint160(address(consensus))))
        );

        // test data
        // no need to test decodedApplication
        (
            address decodedDappOwner,
            bytes32 decodedTemplateHash,
            address decodedApplication
        ) = abi.decode(entry.data, (address, bytes32, address));
        assertEq(_dappOwner, decodedDappOwner);
        assertEq(_templateHash, decodedTemplateHash);
        assertEq(address(newDapp), decodedApplication);
    }
}