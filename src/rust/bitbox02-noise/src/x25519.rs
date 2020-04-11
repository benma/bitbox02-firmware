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
//! but uses a pluggable random number generator to generate keys.

use core::ops::{Deref, DerefMut};
use noise_protocol::U8Array;
use noise_rust_crypto::sensitive::Sensitive;

pub trait Random32 {
    fn mcu_32_bytes(out: &mut [u8; 32]);
}

pub struct X25519<R: Random32>(core::marker::PhantomData<R>);

pub type PrivateKey = [u8; 32];
pub type PublicKey = [u8; 32];

impl<R: Random32> noise_protocol::DH for X25519<R> {
    type Key = Sensitive<PrivateKey>;
    type Pubkey = PublicKey;
    type Output = [u8; 32];

    fn name() -> &'static str {
        "25519"
    }

    fn genkey() -> Self::Key {
        let mut k = Self::Key::new();
        R::mcu_32_bytes(k.deref_mut());
        k[0] &= 248;
        k[31] &= 127;
        k[31] |= 64;
        k
    }

    fn pubkey(k: &Self::Key) -> Self::Pubkey {
        let static_secret = x25519_dalek::StaticSecret::from(*k.deref());
        *x25519_dalek::PublicKey::from(&static_secret).as_bytes()
    }

    fn dh(k: &Self::Key, pk: &Self::Pubkey) -> Result<Self::Output, ()> {
        let k = x25519_dalek::StaticSecret::from(*k.deref());
        let pk = x25519_dalek::PublicKey::from(*pk);
        Ok(*k.diffie_hellman(&pk).as_bytes())
    }
}
