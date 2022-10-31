// Copyright 2022 Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

import { expect, use } from "chai";
import { deployments, ethers } from "hardhat";
import { solidity } from "ethereum-waffle";
import { Signer } from "ethers";
import { keccak256, toUtf8Bytes } from "ethers/lib/utils";
import {
    DebugFacet,
    DebugFacet__factory,
    IPFSFacet,
    IPFSFacet__factory,
    InputFacet,
    InputFacet__factory,
} from "../src/types";
import { deployDiamond, getInputHash } from "./utils";

use(solidity);

describe("IPFS Facet", async () => {
    let signer: Signer;
    let ipfsFacet: IPFSFacet;
    //let debugFacet: DebugFacet;
    //let inputFacet: InputFacet;
    let numberOfInputs = 0x1; // the machine starts with one input

    // Constants
    const INPUT_HEADER = keccak256(toUtf8Bytes("IPFS_addFile"));

    // Test parameters
    const fileName = "example.txt";
    const fileData = ethers.utils.hexlify(
        ethers.utils.toUtf8Bytes("Hello World!")
    );

    beforeEach(async () => {
        await deployments.fixture();

        const diamond = await deployDiamond({ debug: true });
        [signer] = await ethers.getSigners();
        ipfsFacet = IPFSFacet__factory.connect(diamond.address, signer);
        //debugFacet = DebugFacet__factory.connect(diamond.address, signer);
        //inputFacet = InputFacet__factory.connect(diamond.address, signer);
    });

    it("addIPFSFile should return the return value of LibInput.addInput()", async () => {
        const sender = await signer.getAddress();

        // Encode input using the default ABI
        const input = ethers.utils.defaultAbiCoder.encode(
            ["bytes32", "address", "string", "bytes"],
            [INPUT_HEADER, sender, fileName, fileData]
        );

        // Calculate the input hash
        const block = await ethers.provider.getBlock("latest");
        const inputHash = getInputHash(
            input,
            ipfsFacet.address,
            block.number,
            block.timestamp,
            0x0,
            numberOfInputs
        );

        expect(
            await ipfsFacet.callStatic.addIPFSFile(fileName, fileData),
            "addIPFSFile to check return value"
        ).to.equal(inputHash);
    });

    it("addIPFSFile should emit events", async () => {
        expect(
            await ipfsFacet.addIPFSFile(fileName, fileData),
            "expect addIPFSFile function to emit IPFSFileAdded event"
        )
            .to.emit(ipfsFacet, "IPFSFileAdded")
            .withArgs(fileName, (fileData.length - 2) / 2);
    });

    it("addIPFSFile empty name should revert", async () => {
        const fileName = "";
        const fileData = ethers.utils.hexlify(ethers.utils.toUtf8Bytes(""));

        await expect(
            ipfsFacet.addIPFSFile(fileName, fileData),
            "expect addIPFSFile function to revert"
        ).to.be.revertedWith("IPFSFacet: fileName is empty");
    });

    it("addIPFSFile empty file should revert", async () => {
        const fileName = "example.txt";
        const fileData = ethers.utils.hexlify(ethers.utils.toUtf8Bytes(""));

        await expect(
            ipfsFacet.addIPFSFile(fileName, fileData),
            "expect addIPFSFile function to revert"
        ).to.be.revertedWith("IPFSFacet: fileData is empty");
    });
});
