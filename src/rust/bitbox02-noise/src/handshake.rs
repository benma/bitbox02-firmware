// Copyright 2020 Shift Cryptosecurity AG
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

//! This module implements the X25519 trait needed by noise_protocol
//! by using the x25519_dalek crate. It is adapted from
//! https://github.com/sopium/noise-rust/blob/2e4bad0dbe3c674d4e1820e2325ad108c1150dfb/noise-rust-crypto/src/lib.rs#L29,
//! but uses the BitBox02 random number generator to generate keys.

use noise_rust_crypto::{ChaCha20Poly1305, Sha256};
use x25519_dalek::{PublicKey, StaticSecret};

pub enum X25519 {}

type StaticPrivateKey = [u8; 32];

impl noise_protocol::DH for X25519 {
    type Key = StaticPrivateKey;
    type Pubkey = [u8; 32];
    type Output = [u8; 32];

    fn name() -> &'static str {
        "25519"
    }

    fn genkey() -> Self::Key {
        let mut k = [0u8; 32];
        bitbox02::random::mcu_32_bytes(&mut k);
        k[0] &= 248;
        k[31] &= 127;
        k[31] |= 64;
        k
    }

    fn pubkey(k: &Self::Key) -> Self::Pubkey {
        let static_secret = StaticSecret::from(*k);
        *PublicKey::from(&static_secret).as_bytes()
    }

    fn dh(k: &Self::Key, pk: &Self::Pubkey) -> Result<Self::Output, ()> {
        let k = StaticSecret::from(*k);
        let pk = PublicKey::from(*pk);
        Ok(*k.diffie_hellman(&pk).as_bytes())
    }
}
