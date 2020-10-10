// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(not(feature = "app-ethereum"))]
compile_error!(
    "Ethereum code is being compiled even though the app-ethereum feature is not enabled"
);

mod pubrequest;

use super::pb;
use super::Error;

use pb::eth_request::Request;
use pb::eth_response::Response as ETHResponse;
use pb::response::Response;

async fn process_api(request: &Request) -> Option<Result<ETHResponse, Error>> {
    match request {
        Request::Pub(ref request) => Some(pubrequest::process(request).await),
        _ => None,
    }
}

// a `None` result means that the call will fall back to C.
pub async fn process(request: &Request) -> Option<Result<Response, Error>> {
    process_api(request)
        .await
        .map(|r| r.map(|r| Response::Eth(pb::EthResponse { response: Some(r) })))
}
