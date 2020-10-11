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

//! Stubs for testing.

pub use bitbox02_sys::xpub_type_t;

extern crate alloc;
use alloc::string::String;

use crate::password::Password;

pub const BIP39_WORDLIST_LEN: u16 = bitbox02_sys::BIP39_WORDLIST_LEN as u16;
pub const EC_PUBLIC_KEY_UNCOMPRESSED_LEN: usize = bitbox02_sys::EC_PUBLIC_KEY_UNCOMPRESSED_LEN as _;

pub fn is_locked() -> bool {
    panic!("not implemented")
}

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword { remaining_attempts: u8 },
    Unknown,
}

pub fn unlock(_password: &Password) -> Result<(), Error> {
    panic!("not implemented")
}

pub fn unlock_bip39(_mnemonic_passphrase: &Password) -> Result<(), Error> {
    panic!("not implemented")
}

pub fn create_and_store_seed(_password: &Password, _host_entropy: &[u8; 32]) -> bool {
    panic!("not implemented")
}

pub fn get_bip39_mnemonic() -> Result<zeroize::Zeroizing<String>, ()> {
    panic!("not implemented")
}

pub fn get_bip39_word(_idx: u16) -> Result<&'static str, ()> {
    panic!("not implemented")
}

pub fn secp256k1_pubkey_uncompressed(
    _keypath: &[u32],
) -> Result<[u8; EC_PUBLIC_KEY_UNCOMPRESSED_LEN], ()> {
    panic!("not implemented")
}

pub fn encode_xpub_at_keypath(keypath: &[u32], xpub_type: xpub_type_t) -> Result<String, ()> {
    let data = crate::testing::DATA.0.borrow();
    data.keystore_encode_xpub_at_keypath.as_ref().unwrap()(keypath, xpub_type)
}
