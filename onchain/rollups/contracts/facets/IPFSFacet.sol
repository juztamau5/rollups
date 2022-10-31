// Copyright 2022 Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

/// @title IPFS Portal facet
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Context.sol";

import {IIPFS} from "../interfaces/IIPFS.sol";

import {LibInput} from "../libraries/LibInput.sol";

contract IPFSFacet is Context, IIPFS {
    using LibInput for LibInput.DiamondStorage;

    //////////////////////////////////////////////////////////////////////////////
    // Constants
    //////////////////////////////////////////////////////////////////////////////

    bytes32 constant INPUT_HEADER_addIPFSFile = keccak256("IPFS_addFile");

    //////////////////////////////////////////////////////////////////////////////
    // Events
    //////////////////////////////////////////////////////////////////////////////

    event IPFSFileAdded(string fileName, uint256 fileSize);

    //////////////////////////////////////////////////////////////////////////////
    // Implementation of {IIPFS}
    //////////////////////////////////////////////////////////////////////////////

    /**
     * @dev See {IIPFS-addIPFSFile}
     */
    function addIPFSFile(string memory fileName, bytes memory fileData)
        public
        override
        returns (bytes32)
    {
        // Validate parameters
        require(bytes(fileName).length > 0, "IPFSFacet: fileName is empty");
        require(fileData.length > 0, "IPFSFacet: fileData is empty");

        LibInput.DiamondStorage storage inputDS = LibInput.diamondStorage();

        bytes memory input = abi.encode(
            INPUT_HEADER_addIPFSFile,
            _msgSender(),
            fileName,
            fileData
        );

        emit IPFSFileAdded(fileName, fileData.length);
        return inputDS.addInternalInput(input);
    }
}
