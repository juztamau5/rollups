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

mod common;
use crate::common::*;

struct EchoInspect {}

#[tonic::async_trait]
impl MockInspect for EchoInspect {
    async fn inspect_state(&self, payload: Vec<u8>) -> MockInspectResponse {
        MockInspectResponse {
            reports: vec![Report { payload }],
            exception: None,
            completion_status: CompletionStatus::Accepted,
        }
    }
}

async fn test_payload(sent_payload: &str, expected_payload: &str) {
    let test_state = TestState::setup(EchoInspect {}).await;
    let response = send_request(sent_payload)
        .await
        .expect("failed to obtain response");
    assert_eq!(response.status, "Accepted");
    assert_eq!(response.exception_payload, None);
    assert_eq!(response.reports.len(), 1);
    let expected_payload = String::from("0x") + &hex::encode(expected_payload);
    assert_eq!(response.reports[0].payload, expected_payload);
    test_state.teardown().await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_simple_payload() {
    test_payload("hello", "hello").await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_payload_with_spaces() {
    test_payload("hello world", "hello world").await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_url_encoded_payload() {
    test_payload("hello%20world", "hello world").await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_payload_with_slashes() {
    test_payload("user/123/name", "user/123/name").await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_payload_with_path_and_query() {
    test_payload(
        "user/data?key=value&key2=value2",
        "user/data?key=value&key2=value2",
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_raw_json_payload() {
    test_payload(
        r#"{"key": ["value1", "value2"]}"#,
        r#"{"key": ["value1", "value2"]}"#,
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_payload_with_empty_payload() {
    test_payload("", "").await;
}
