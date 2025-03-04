// Copyright Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

/// @title Test base contract
pragma solidity ^0.8.8;

import {Test} from "forge-std/Test.sol";

contract TestBase is Test {
    /// @notice Guarantess `addr` is an address that can be mocked
    /// @dev Some addresses are reserved by Forge and should not be mocked
    modifier isMockable(address addr) {
        vm.assume(addr != VM_ADDRESS);
        vm.assume(addr != CONSOLE);
        vm.assume(addr != DEFAULT_SENDER);
        vm.assume(addr != DEFAULT_TEST_CONTRACT);
        vm.assume(addr != MULTICALL3_ADDRESS);
        _;
    }
}
