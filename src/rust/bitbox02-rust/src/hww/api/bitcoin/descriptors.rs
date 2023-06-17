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

use pb::btc_script_config::descriptor::key::Key;
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

// Arbitrary limit of keys that can be present in a descriptor.
const MAX_KEYS: usize = 20;

// We only support Bitcoin testnet for now.
fn check_enabled(coin: BtcCoin) -> Result<(), Error> {
    if !matches!(coin, BtcCoin::Tbtc) {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

fn is_our_key(key: &Key) -> Result<bool, ()> {
    let our_root_fingerprint = crate::keystore::root_fingerprint()?;
    match key {
        Key::KeyOriginInfo(pb::KeyOriginInfo {
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
            Key::KeyOriginInfo(pb::KeyOriginInfo {
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

fn hash_key(key: &pb::btc_script_config::descriptor::Key) -> Result<Vec<u8>, ()> {
    let mut hasher = Sha256::new();
    match key.key.as_ref() {
        Some(Key::KeyOriginInfo(pb::KeyOriginInfo {
            xpub: Some(xpub), ..
        })) => {
            // hash key type in case we add other key types in the future.
            let key_type: u8 = 0;
            hasher.update(key_type.to_le_bytes());
            hasher.update(&bip32::Xpub::from(xpub).serialize(None)?);
        }
        _ => return Err(()),
    }
    Ok(hasher.finalize().as_slice().into())
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
    {
        // 3. descriptor
        let len: u32 = descriptor.descriptor.len() as _;
        hasher.update(len.to_le_bytes());
        hasher.update(descriptor.descriptor.as_bytes());
    }
    {
        // 4. keys
        let num: u32 = descriptor.keys.len() as _;
        hasher.update(num.to_le_bytes());
        let keys_hashed: Vec<Vec<u8>> = descriptor
            .keys
            .iter()
            .map(hash_key)
            .collect::<Result<Vec<Vec<u8>>, ()>>()?;
        for key_hash in keys_hashed.iter() {
            hasher.update(key_hash);
        }
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
                    Some(Key::KeyOriginInfo(pb::KeyOriginInfo {
                        xpub: Some(xpub), ..
                    })),
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
    let (left, right) = pk.strip_prefix('@').ok_or(())?.split_once('/').ok_or(())?;
    validate_no_leading_zero(left)?;
    let (receive_index, change_index): (u32, u32) = match right {
        "**" => (0, 1),
        right => {
            let (left_number_str, right_number_str) = right
                .strip_prefix('<')
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

// Provides two modes of derivations for the descriptor. Either directly using the multipath index
// and address index, or a full keypath at one of our keys. See the docs in the variants for examples.
pub enum Derive<'a> {
    // Dervive descriptor using (multpath index, address index), where the multipath index can be 0
    // or 1 (selecting between "<left;right>".
    // Example: wsh(and_v(v:pk(@0/<10;11>/*),pk(@1/<20;21>/*))) derived using (0,5) derives:
    // wsh(and_v(v:pk(@0/10/5),pk(@1/20/5))).
    // The same derived usin (1,5) derives: wsh(and_v(v:pk(@0/11/5),pk(@1/21/5)))
    Direct(u32, u32),
    // Derive descriptor at the "keypath".
    // Example: wsh(and_v(v:pk(@0/<10;11>/*),pk(@1/<20;21>/*))) with our key [fp/48'/1'/0'/3']xpub...]
    // derived using keypath m/48'/1'/0'/3'/11/5 derives:
    // wsh(and_v(v:pk(@0/11/5),pk(@1/21/5))).
    Keypath(&'a [u32]),
}

fn get_multipath_and_address_index(
    miniscript_expr: &miniscript::Miniscript<String, miniscript::Segwitv0>,
    derivation: Derive,
    keys: &[pb::btc_script_config::descriptor::Key],
) -> Result<(u32, u32), Error> {
    match derivation {
        Derive::Direct(multipath_index, address_index) => Ok((multipath_index, address_index)),
        Derive::Keypath(keypath) => {
            for pk in miniscript_expr.iter_pk() {
                let (key_index, multipath_index_left, multipath_index_right) =
                    parse_wallet_policy_pk(&pk).or(Err(Error::InvalidInput))?;

                match keys.get(key_index) {
                    Some(pb::btc_script_config::descriptor::Key {
                        key:
                            Some(Key::KeyOriginInfo(pb::KeyOriginInfo {
                                keypath: keypath_account,
                                ..
                            })),
                    }) if keypath.starts_with(&keypath_account)
                        && keypath.len() == keypath_account.len() + 2 =>
                    {
                        let keypath_change = keypath[keypath.len() - 2];
                        let mp = if keypath_change == multipath_index_left {
                            0
                        } else if keypath_change == multipath_index_right {
                            1
                        } else {
                            continue;
                        };
                        return Ok((mp, keypath[keypath.len() - 1]));
                    }
                    _ => continue,
                }
            }
            Err(Error::InvalidInput)
        }
    }
}

pub struct ParsedDescriptor<'a> {
    pub descriptor: &'a Descriptor,
    pub miniscript_expr: miniscript::Miniscript<String, miniscript::Segwitv0>,
    pub output_type: BtcOutputType,
}

impl<'a> ParsedDescriptor<'a> {
    /// Check that it is impossible to create a derivation with duplicate pubkeys, assuming all the keys in the key vector are distinct.
    ///
    /// Even though the rust-miniscript library checks for duplicate keys, it does so on the raw
    /// miniscript, which would not catch e.g. that `wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<2;1>/*)))` has
    /// a duplicate change derivation if we derive at the receive path.
    ///
    /// Also checks that each key is used, e.g. if there are 3 keys in the key vector, @0, @1 and @2
    /// must be present.
    fn validate_keys(&self) -> Result<(), Error> {
        // in "@key_index/<left;right>", keeps track of (key_index,left) and (key_index,right) to check
        // for duplicates.
        let mut derivations_seen: Vec<(usize, u32)> = Vec::new();

        let mut keys_seen: Vec<bool> = vec![false; self.descriptor.keys.len()];

        for pk in self.miniscript_expr.iter_pk() {
            let (key_index, multipath_index_left, multipath_index_right) =
                parse_wallet_policy_pk(&pk).or(Err(Error::InvalidInput))?;

            if derivations_seen.contains(&(key_index, multipath_index_left)) {
                return Err(Error::InvalidInput);
            }
            derivations_seen.push((key_index, multipath_index_left));
            if derivations_seen.contains(&(key_index, multipath_index_right)) {
                return Err(Error::InvalidInput);
            }
            derivations_seen.push((key_index, multipath_index_right));

            *keys_seen.get_mut(key_index).ok_or(Error::InvalidInput)? = true;
        }

        if !keys_seen.into_iter().all(|b| b) {
            return Err(Error::InvalidInput);
        }
        Ok(())
    }

    /// Validate checks that a descriptor config is valid:
    /// - At least one key is ours
    /// - No two keys are the same.
    /// - TOOD: document more
    pub fn validate(&self, coin: BtcCoin) -> Result<(), Error> {
        check_enabled(coin)?;

        // Already called as part from parsing miniscript using from_str.
        // self.miniscript_expr
        //     .sanity_check()
        //     .or(Err(Error::InvalidInput))?;

        // Check that all keys are provided (no empty elements in the list).
        let keys: Vec<&Key> = self
            .descriptor
            .keys
            .iter()
            .map(|key| key.key.as_ref())
            .collect::<Option<Vec<&Key>>>()
            .ok_or(Error::InvalidInput)?;

        if keys.len() > MAX_KEYS {
            return Err(Error::InvalidInput);
        }

        self.validate_keys()?;

        // Check that at least one key is ours.
        let has_our_key = 'block: {
            for &key in keys.iter() {
                if is_our_key(key)? {
                    break 'block true;
                }
            }
            false
        };
        if !has_our_key {
            return Err(Error::InvalidInput);
        }

        // Check for duplicate xpubs.
        // Extract all xpubs first.
        let xpubs: Vec<&pb::XPub> = keys
            .iter()
            .filter_map(|key| match key {
                Key::KeyOriginInfo(pb::KeyOriginInfo {
                    xpub: Some(xpub), ..
                }) => Some(xpub),
                _ => None,
            })
            .collect();
        if (1..xpubs.len()).any(|i| xpubs[i..].contains(&xpubs[i - 1])) {
            return Err(Error::InvalidInput);
        }

        Ok(())
    }

    pub fn pkscript(&self, derivation: Derive) -> Result<ParseResult, Error> {
        let (multipath_index, address_index) = get_multipath_and_address_index(
            &self.miniscript_expr,
            derivation,
            &self.descriptor.keys,
        )?;
        let mut translator = WalletPolicyPkTranslator {
            keys: self.descriptor.keys.as_ref(),
            multipath_index,
            address_index,
        };
        let miniscript_expr = self.miniscript_expr.translate_pk(&mut translator)?;
        Ok(ParseResult {
            pkscript: miniscript_expr.encode().as_bytes().to_vec(),
            output_type: self.output_type,
        })
    }
}

pub fn parse(descriptor: &Descriptor) -> Result<ParsedDescriptor, Error> {
    let desc = descriptor.descriptor.as_str();
    match desc.as_bytes() {
        [b'w', b's', b'h', b'(', .., b')'] => {
            let miniscript_expr: miniscript::Miniscript<String, miniscript::Segwitv0> =
                miniscript::Miniscript::from_str(&desc[4..desc.len() - 1])
                    .or(Err(Error::InvalidInput))?;

            Ok(ParsedDescriptor {
                descriptor,
                miniscript_expr,
                output_type: BtcOutputType::P2wsh,
            })
        }
        _ => Err(Error::InvalidInput),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bip32::parse_xpub;
    use bitbox02::testing::{mock_memory, mock_unlocked};
    use util::bip32::HARDENED;

    #[test]
    fn test_parse_wallet_policy_pk() {
        assert_eq!(parse_wallet_policy_pk("@0/**"), Ok((0, 0, 1)));
        assert_eq!(parse_wallet_policy_pk("@1/**"), Ok((1, 0, 1)));
        assert_eq!(parse_wallet_policy_pk("@100/**"), Ok((100, 0, 1)));

        assert_eq!(parse_wallet_policy_pk("@0/<0;1>/*"), Ok((0, 0, 1)));
        assert_eq!(parse_wallet_policy_pk("@0/<1;2>/*"), Ok((0, 1, 2)));
        assert_eq!(parse_wallet_policy_pk("@0/<100;101>/*"), Ok((0, 100, 101)));
        assert_eq!(
            parse_wallet_policy_pk("@50/<100;101>/*"),
            Ok((50, 100, 101))
        );

        assert!(parse_wallet_policy_pk("@00/**").is_err());
        assert!(parse_wallet_policy_pk("@01/**").is_err());
        assert!(parse_wallet_policy_pk("@0").is_err());
        assert!(parse_wallet_policy_pk("@0/").is_err());
        assert!(parse_wallet_policy_pk("@0/*").is_err());
        assert!(parse_wallet_policy_pk("0/**").is_err());
        assert!(parse_wallet_policy_pk("@-1/**").is_err());
        assert!(parse_wallet_policy_pk("@0/<0;1>/*/*").is_err());
        assert!(parse_wallet_policy_pk("@0/<0;1>").is_err());
        assert!(parse_wallet_policy_pk("@0/<0;1>/").is_err());
        assert!(parse_wallet_policy_pk("@0/<100;100>/*").is_err());
        // 2147483648 = HARDENED offset.
        assert!(parse_wallet_policy_pk("@0/<100;2147483648>/*").is_err());
        assert!(parse_wallet_policy_pk("@0/<2147483648;100>/*").is_err());
    }

    // Creates a descriptor for one of our own keys at keypath.
    fn make_our_key(keypath: &[u32]) -> pb::btc_script_config::descriptor::Key {
        let our_xpub = crate::keystore::get_xpub(keypath).unwrap();
        pb::btc_script_config::descriptor::Key {
            key: Some(Key::KeyOriginInfo(pb::KeyOriginInfo {
                root_fingerprint: crate::keystore::root_fingerprint().unwrap(),
                keypath: keypath.to_vec(),
                xpub: Some(our_xpub.into()),
            })),
        }
    }

    // Creates a descriptor key without fingerprint/keypath from an xpub string.
    fn make_key(xpub: &str) -> pb::btc_script_config::descriptor::Key {
        pb::btc_script_config::descriptor::Key {
            key: Some(Key::KeyOriginInfo(pb::KeyOriginInfo {
                root_fingerprint: vec![],
                keypath: vec![],
                xpub: Some(parse_xpub(xpub).unwrap()),
            })),
        }
    }

    const SOME_XPUB_1: &str = "xpub6FMWuwbCA9KhoRzAMm63ZhLspk5S2DM5sePo8J8mQhcS1xyMbAqnc7Q7UescVEVFCS6qBMQLkEJWQ9Z3aDPgBov5nFUYxsJhwumsxM4npSo";

    fn make_descriptor(
        descriptor: &str,
        keys: &[pb::btc_script_config::descriptor::Key],
    ) -> Descriptor {
        Descriptor {
            descriptor: descriptor.into(),
            keys: keys.to_vec(),
        }
    }

    #[test]
    fn test_get_multipath_and_address_index() {
        let make_expr = |s: &str| miniscript::Miniscript::from_str(s).unwrap();

        assert_eq!(
            get_multipath_and_address_index(&make_expr("pk(@0/**)"), Derive::Direct(1, 12), &[]),
            Ok((1, 12)),
        );

        let keypath_account = &[48 + HARDENED, 1 + HARDENED, 0 + HARDENED, 3 + HARDENED];
        assert_eq!(
            get_multipath_and_address_index(
                &make_expr("and_v(v:pk(@0/**),pk(@1/**))"),
                Derive::Keypath(&[
                    48 + HARDENED,
                    1 + HARDENED,
                    0 + HARDENED,
                    3 + HARDENED,
                    0,
                    12,
                ]),
                &[make_key(SOME_XPUB_1), make_our_key(keypath_account)]
            ),
            Ok((0, 12)),
        );
        assert_eq!(
            get_multipath_and_address_index(
                &make_expr("and_v(v:pk(@0/**),pk(@1/**))"),
                Derive::Keypath(&[
                    48 + HARDENED,
                    1 + HARDENED,
                    0 + HARDENED,
                    3 + HARDENED,
                    1,
                    12,
                ]),
                &[make_key(SOME_XPUB_1), make_our_key(keypath_account)]
            ),
            Ok((1, 12)),
        );

        assert_eq!(
            get_multipath_and_address_index(
                &make_expr("and_v(v:pk(@0/**),pk(@1/<10;11>/*))"),
                Derive::Keypath(&[
                    48 + HARDENED,
                    1 + HARDENED,
                    0 + HARDENED,
                    3 + HARDENED,
                    10,
                    12,
                ]),
                &[make_key(SOME_XPUB_1), make_our_key(keypath_account)]
            ),
            Ok((0, 12)),
        );
        assert_eq!(
            get_multipath_and_address_index(
                &make_expr("and_v(v:pk(@0/**),pk(@1/<10;11>/*))"),
                Derive::Keypath(&[
                    48 + HARDENED,
                    1 + HARDENED,
                    0 + HARDENED,
                    3 + HARDENED,
                    11,
                    12,
                ]),
                &[make_key(SOME_XPUB_1), make_our_key(keypath_account)]
            ),
            Ok((1, 12)),
        );
    }

    #[test]
    fn test_parse_check_dups_in_descriptor() {
        mock_unlocked();
        let coin = BtcCoin::Tbtc;
        let keypath = &[48 + HARDENED, 1 + HARDENED, 0 + HARDENED, 3 + HARDENED];

        // Ok, one key.
        let desc = make_descriptor("wsh(pk(@0/**))", &[make_our_key(keypath)]);
        assert!(parse(&desc).unwrap().validate(coin).is_ok());

        // Ok, two keys.
        let desc = make_descriptor(
            "wsh(or_b(pk(@0/**),s:pk(@1/**)))",
            &[make_our_key(keypath), make_key(SOME_XPUB_1)],
        );
        assert!(parse(&desc).unwrap().validate(coin).is_ok());

        // Ok, one key with different derivations
        let desc = make_descriptor(
            "wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<2;3>/*)))",
            &[make_our_key(keypath)],
        );
        assert!(parse(&desc).unwrap().validate(coin).is_ok());

        // Duplicate path, one time in change, one time in receive. While the keys technically are
        // never duplicate in the final miniscript with the pubkeys inserted, we still prohibit, as
        // it does not look like there would be a sane use case for this and would likely be an
        // accident.
        let desc = make_descriptor(
            "wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<1;2>/*)))",
            &[make_our_key(keypath)],
        );
        assert!(parse(&desc).unwrap().validate(coin).is_err());

        // Duplicate key inside descriptor.
        let desc = make_descriptor("wsh(or_b(pk(@0/**),s:pk(@0/**)))", &[make_our_key(keypath)]);
        assert!(parse(&desc).is_err());

        // Duplicate key inside descriptor (same change and receive).
        let desc = make_descriptor("wsh(pk(@0/<0;0>/*))", &[make_our_key(keypath)]);
        assert!(parse(&desc).unwrap().validate(coin).is_err());

        // Duplicate key inside descriptor, using different notations for the same thing.
        let desc = make_descriptor(
            "wsh(or_b(pk(@0/**),s:pk(@0/<0;1>/*)))",
            &[make_our_key(keypath)],
        );
        assert!(parse(&desc).unwrap().validate(coin).is_err());

        // Duplicate key inside descriptor, using same receive but different change.
        let desc = make_descriptor(
            "wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<0;2>/*)))",
            &[make_our_key(keypath)],
        );
        assert!(parse(&desc).unwrap().validate(coin).is_err());

        // Duplicate key inside descriptor, using same change but different receive.
        let desc = make_descriptor(
            "wsh(or_b(pk(@0/<0;1>/*),s:pk(@0/<2;1>/*)))",
            &[make_our_key(keypath)],
        );
        assert!(parse(&desc).unwrap().validate(coin).is_err());
    }
}
