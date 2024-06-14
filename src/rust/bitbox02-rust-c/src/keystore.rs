// Copyright 2024 Shift Crypto AG
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

#[cfg(feature = "app-bitcoin")]
#[no_mangle]
pub extern "C" fn rust_keystore_schnorr_bip86_tweak_private_key(
    private_key: crate::util::Bytes,
    mut tweaked_private_key: crate::util::BytesMut,
) -> bool {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let untweaked =
        match bitcoin::key::UntweakedKeypair::from_seckey_slice(&secp, private_key.as_ref()) {
            Ok(k) => k,
            Err(_) => return false,
        };
    use bitcoin::key::TapTweak;
    let tweaked = untweaked.tap_tweak(&secp, None).to_inner();
    tweaked_private_key
        .as_mut()
        .copy_from_slice(&tweaked.secret_bytes());
    // let (xonly_pubkey, _) = tweaked.x_only_public_key();
    // tweaked_public_key
    //     .as_mut()
    //     .copy_from_slice(&xonly_pubkey.serialize());
    true
}
