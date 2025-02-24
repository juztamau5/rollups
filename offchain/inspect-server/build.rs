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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &[
                "../../grpc-interfaces/versioning.proto",
                "../../grpc-interfaces/server-manager.proto",
            ],
            &["../../grpc-interfaces"],
        )?;
    println!("cargo:rerun-if-changed=../../grpc-interfaces/versioning.proto");
    println!(
        "cargo:rerun-if-changed=../../grpc-interfaces/server-manager.proto"
    );
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
