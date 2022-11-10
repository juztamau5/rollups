// Copyright 2022 Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

/// @title Output Encoding Library
pragma solidity ^0.8.13;

library OutputEncoding {
    /// @notice An arbitrary blob of data
    /// @dev It can be verified by anyone and any number of times
    bytes1 constant NOTICE = hex"00";

    /// @notice An arbitrary L1 function call
    /// @dev It can be executed by anyone and at most once
    bytes1 constant VOUCHER = hex"01";

    /// @notice Encode notice
    /// @param notice The notice
    /// @return The encoded output
    function encodeNotice(bytes calldata notice)
        internal
        pure
        returns (bytes memory)
    {
        return
            abi.encodePacked(
                NOTICE, // 1B
                notice //  arbitrary size
            );
    }

    /// @notice Encode voucher
    /// @param destination The contract that will execute the payload
    /// @param payload The ABI-encoded function call
    /// @return The encoded output
    function encodeVoucher(address destination, bytes calldata payload)
        internal
        pure
        returns (bytes memory)
    {
        return
            abi.encodePacked(
                VOUCHER, //     1B
                destination, // 20B
                payload //      arbitrary size
            );
    }
}