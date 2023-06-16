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
pub use pb::{BtcCoin, BtcOutputType};

use super::params::Params;

use crate::bip32;
use crate::workflow::confirm;
use util::bip32::HARDENED;

use core::str::FromStr;

use alloc::string::String;
use alloc::vec::Vec;

use miniscript::TranslatePk;

use sha2::{Digest, Sha256};

// We only support Bitcoin testnet for now.
fn check_enabled(coin: BtcCoin) -> Result<(), Error> {
    if !matches!(coin, BtcCoin::Tbtc) {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

fn is_our_key(key: &pb::btc_script_config::descriptor::key::Key) -> Result<bool, ()> {
    let our_root_fingerprint = crate::keystore::root_fingerprint()?;
    match key {
        pb::btc_script_config::descriptor::key::Key::KeyOriginInfo(pb::KeyOriginInfo {
            root_fingerprint,
            keypath,
            xpub: Some(xpub),
            ..
        }) if root_fingerprint.as_slice() == our_root_fingerprint.as_slice() => {
            let our_xpub = crate::keystore::get_xpub(keypath)?.serialize(None)?;
            let maybe_our_xpub = bip32::Xpub::from(xpub).serialize(None)?;
            Ok(our_xpub == maybe_our_xpub)
        }
        _ => Ok(false),
    }
}

/// Validate checks that a descriptor config is valid:
/// - At least one key is ours
/// - No two keys are the same.
/// - TOOD: document more
pub fn validate(coin: BtcCoin, descriptor: &Descriptor) -> Result<(), Error> {
    check_enabled(coin)?;
    // Check that all keys are provided (no empty elements in the list).
    let keys: Vec<&pb::btc_script_config::descriptor::key::Key> = descriptor
        .keys
        .iter()
        .map(|key| key.key.as_ref())
        .collect::<Option<Vec<&pb::btc_script_config::descriptor::key::Key>>>()
        .ok_or(Error::InvalidInput)?;

    // Check that at least one key is ours.
    let mut has_our_key = false;
    for &key in keys.iter() {
        if is_our_key(key)? {
            has_our_key = true;
            break;
        }
    }
    if !has_our_key {
        return Err(Error::InvalidInput);
    }

    // Check for duplicate xpubs.
    // Extract all xpubs first.
    let xpubs: Vec<&pb::XPub> = keys
        .iter()
        .filter_map(|key| match key {
            pb::btc_script_config::descriptor::key::Key::KeyOriginInfo(pb::KeyOriginInfo {
                xpub: Some(xpub),
                ..
            }) => Some(xpub),
            _ => None,
        })
        .collect();
    if (1..xpubs.len()).any(|i| xpubs[i..].contains(&xpubs[i - 1])) {
        return Err(Error::InvalidInput);
    }

    parse(descriptor, 0, 0)?;

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
        title,
        body: &descriptor.descriptor,
        scrollable: true,
        accept_is_nextarrow: true,
        ..Default::default()
    })
    .await?;

    let num_keys = descriptor.keys.len();
    for (i, key) in descriptor.keys.iter().enumerate() {
        let key = key.key.as_ref().ok_or(Error::InvalidInput)?;
        let key_str = match key {
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
            title: &format!("Key {}/{}", i + 1, num_keys),
            body: (if is_our_key(key)? {
                format!("This device: {}", key_str)
            } else {
                key_str
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
    // TODO: also hash `parse(descriptor, 1, 0)` to cover all multipaths
    let parse_result = parse(descriptor, 0, 0).or(Err(()))?;
    {
        // 3. adress type
        let address_type: u32 = parse_result.output_type as _;
        hasher.update(address_type.to_le_bytes());
    }
    {
        // 4. pkscript of first address.
        let len: u32 = parse_result.pkscript.len() as _;
        hasher.update(len.to_le_bytes());
        hasher.update(&parse_result.pkscript);
    }
    Ok(hasher.finalize().as_slice().into())
}

pub fn get_name(coin: BtcCoin, descriptor: &Descriptor) -> Result<Option<String>, ()> {
    Ok(bitbox02::memory::multisig_get_by_hash(&get_hash(
        coin, descriptor,
    )?))
}

struct WalletPolicyPkTranslator<'a> {
    keys: &'a [pb::btc_script_config::descriptor::Key],
    multipath_index: u32,
    address_index: u32,
}

impl<'a> miniscript::Translator<String, bitcoin::PublicKey, Error>
    for WalletPolicyPkTranslator<'a>
{
    fn pk(&mut self, pk: &String) -> Result<bitcoin::PublicKey, Error> {
        let (key_index, multipath_index_left, multipath_index_right) =
            parse_wallet_policy_pk(&pk).or(Err(Error::InvalidInput))?;
        match self.keys.get(key_index) {
            Some(pb::btc_script_config::descriptor::Key {
                key:
                    Some(pb::btc_script_config::descriptor::key::Key::KeyOriginInfo(
                        pb::KeyOriginInfo {
                            xpub: Some(xpub), ..
                        },
                    )),
            }) => {
                let xpub: crate::bip32::Xpub = xpub.into();
                let multipath_index = match self.multipath_index {
                    0 => multipath_index_left,
                    1 => multipath_index_right,
                    _ => return Err(Error::InvalidInput),
                };
                let xpub = xpub.derive(&[multipath_index, self.address_index])?;
                Ok(bitcoin::PublicKey::from_slice(xpub.public_key()).or(Err(Error::Generic))?)
            }
            _ => Err(Error::InvalidInput),
        }
    }

    miniscript::translate_hash_fail!(String, bitcoin::PublicKey, Error);
}

fn parse_wallet_policy_pk(pk: &str) -> Result<(usize, u32, u32), ()> {
    fn validate_no_leading_zero(num: &str) -> Result<(), ()> {
        if num.len() > 1 && num.starts_with('0') {
            Err(())
        } else {
            Ok(())
        }
    }
    let (left, right) = pk.strip_prefix("@").ok_or(())?.split_once('/').ok_or(())?;
    validate_no_leading_zero(left)?;
    let (receive_index, change_index): (u32, u32) = match right {
        "**" => (0, 1),
        right => {
            let (left_number_str, right_number_str) = right
                .strip_prefix("<")
                .ok_or(())?
                .strip_suffix(">/*")
                .ok_or(())?
                .split_once(';')
                .ok_or(())?;
            validate_no_leading_zero(left_number_str)?;
            validate_no_leading_zero(right_number_str)?;
            (
                left_number_str.parse().or(Err(()))?,
                right_number_str.parse().or(Err(()))?,
            )
        }
    };
    if receive_index == change_index || receive_index >= HARDENED || change_index >= HARDENED {
        return Err(());
    }
    Ok((left.parse().or(Err(()))?, receive_index, change_index))
}

pub struct ParseResult {
    pub pkscript: Vec<u8>,
    pub output_type: BtcOutputType,
}

pub fn parse(
    descriptor: &Descriptor,
    multipath_index: u32,
    address_index: u32,
) -> Result<ParseResult, Error> {
    let desc = descriptor.descriptor.as_str();
    match desc.as_bytes() {
        [b'w', b's', b'h', b'(', .., b')'] => {
            let miniscript_expr: miniscript::Miniscript<String, miniscript::Segwitv0> =
                miniscript::Miniscript::from_str(&desc[4..desc.len() - 1])
                    .or(Err(Error::InvalidInput))?;
            let mut translator = WalletPolicyPkTranslator {
                keys: descriptor.keys.as_ref(),
                multipath_index,
                address_index,
            };
            miniscript_expr
                .sanity_check()
                .or(Err(Error::InvalidInput))?;
            // TODO: check that all keys are used.
            let miniscript_expr = miniscript_expr.translate_pk(&mut translator)?;
            Ok(ParseResult {
                pkscript: miniscript_expr.encode().as_bytes().to_vec(),
                output_type: BtcOutputType::P2wsh,
            })
        }
        _ => Err(Error::InvalidInput),
    }
}
