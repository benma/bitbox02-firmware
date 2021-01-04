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

use super::Error;
use crate::pb;

use pb::response::Response;

use crate::workflow::confirm;

pub async fn process() -> Result<Response, Error> {
    let number = bitbox02::random::bytes32().to_vec();
    let number_hex = hex::encode(&number);
    let request =
        crate::hww::next_request(Response::RandomNumber(pb::RandomNumberResponse { number }))
            .await?;
    if !matches!(
        request,
        pb::request::Request::RandomNumber(pb::RandomNumberRequest {})
    ) {
        return Err(Error::InvalidInput);
    }
    let body = format!(
        "{}\n{}\n{}\n{}",
        &number_hex[..16],
        &number_hex[16..32],
        &number_hex[32..48],
        &number_hex[48..64]
    );
    let params = confirm::Params {
        title: "Random",
        body: &body,
        accept_only: true,
        ..Default::default()
    };
    // Ignore result, can only be true (accept_only).
    confirm::confirm(&params).await;
    Ok(Response::Success(pb::Success {}))
}
