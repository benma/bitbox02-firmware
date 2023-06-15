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
use pb::BtcCoin;

use super::common::Payload;
use super::params::Params;

use crate::bip32;
use crate::workflow::confirm;

use alloc::string::String;
use alloc::vec::Vec;

use sha2::{Digest, Sha256};

// We only support Bitcoin testnet for now.
fn check_enabled(coin: BtcCoin) -> Result<(), Error> {
    if !matches!(coin, BtcCoin::Tbtc) {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

/// Validate checks that a descriptor config is valid:
/// - At least one key is ours
/// - All keys are used
/// - TOOD: document more
pub fn validate(coin: BtcCoin, descriptor: &Descriptor) -> Result<(), Error> {
    check_enabled(coin)?;
    Payload::from_descriptor(descriptor, 0, 0)?;
    Ok(())
}

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

/// Creates a hash of this descriptor config, useful for registration and identification.
pub fn get_hash(coin: BtcCoin, descriptor: &Descriptor) -> Result<Vec<u8>, ()> {
    let mut hasher = Sha256::new();
    {
        // 1. Type of registration: descriptor
        hasher.update(&[0xff]);
    }
    {
        // 2. coin
        let byte: u8 = match coin {
            BtcCoin::Btc => 0x00,
            BtcCoin::Tbtc => 0x01,
            BtcCoin::Ltc => 0x02,
            BtcCoin::Tltc => 0x03,
        };
        hasher.update(byte.to_le_bytes());
    }
    let payload = super::common::Payload::from_descriptor(descriptor, 0, 0).or(Err(()))?;
    {
        // 3. adress type
        let address_type: u32 = payload.output_type as _;
        hasher.update(address_type.to_le_bytes());
    }
    {
        // 4. payload of first address.
        let len: u32 = payload.data.len() as _;
        hasher.update(len.to_le_bytes());
        hasher.update(&payload.data);
    }
    Ok(hasher.finalize().as_slice().into())
}

pub fn get_name(coin: BtcCoin, descriptor: &Descriptor) -> Result<Option<String>, ()> {
    Ok(bitbox02::memory::multisig_get_by_hash(&get_hash(
        coin, descriptor,
    )?))
}
