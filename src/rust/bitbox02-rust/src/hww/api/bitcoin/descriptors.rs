// Copyright 2023 Shift Crypto AG
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

use super::pb;
use super::Error;

use pb::btc_script_config::Descriptor;

use super::params::Params;

use crate::bip32;
use crate::workflow::confirm;

pub enum Mode {
    Basic,
    Advanced,
}

pub async fn confirm(
    title: &str,
    params: &Params,
    name: &str,
    descriptor: &Descriptor,
    mode: Mode,
) -> Result<(), Error> {
    confirm::confirm(&confirm::Params {
        title,
        body: &format!(
            "{}\npolicy with\n{} keys",
            params.name,
            descriptor.keys.len(),
        ),
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    confirm::confirm(&confirm::Params {
        title,
        body: name,
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    if matches!(mode, Mode::Basic) {
        if let Err(confirm::UserAbort) = confirm::confirm(&confirm::Params {
            body: "Show policy\ndetails?",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await
        {
            return Ok(());
        }
    }

    confirm::confirm(&confirm::Params {
        body: &descriptor.descriptor,
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    let num_keys = descriptor.keys.len();
    for (i, key) in descriptor.keys.iter().enumerate() {
        let key_str = match key.key.as_ref().ok_or(Error::InvalidInput)? {
            pb::btc_script_config::descriptor::key::Key::KeyOriginInfo(pb::KeyOriginInfo {
                root_fingerprint,
                keypath,
                xpub: Some(xpub),
            }) => {
                let xpub_str = bip32::Xpub::from(xpub)
                    .serialize_str(bip32::XPubType::Xpub)
                    .or(Err(Error::InvalidInput))?;
                if root_fingerprint.is_empty() {
                    xpub_str
                } else if root_fingerprint.len() != 4 {
                    return Err(Error::InvalidInput);
                } else {
                    format!(
                        "[{}/{}]{}",
                        hex::encode(root_fingerprint),
                        util::bip32::to_string_no_prefix(keypath),
                        xpub_str
                    )
                }
            }
            _ => return Err(Error::InvalidInput),
        };
        confirm::confirm(&confirm::Params {
            body: (if i == descriptor.our_key_index as usize {
                format!("Key {}/{} (this device): {}", i + 1, num_keys, key_str)
            } else {
                format!("Key {}/{}: {}", i + 1, num_keys, key_str)
            })
            .as_str(),
            scrollable: true,
            longtouch: i == num_keys - 1 && matches!(mode, Mode::Advanced),
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    }
    Ok(())
}
