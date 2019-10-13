// Copyright 2019 Shift Cryptosecurity AG
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

#ifndef _KEYSTORE_NONCE_H_
#define _KEYSTORE_NONCE_H_

#include "compiler_util.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/**
 * Calls keystore_secp256k1_nonce_commit, but it caches the keypath and msg32 to be used when
 * signing.
 * Must use `keystore_nonce_clear` after signing or aborting the signing process.
 * @return false if there is previous uncleared cache data or `keystore_secp256k1_nonce_commit`
 * fails.
 */
USE_RESULT bool keystore_nonce_secp256k1_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* client_commitment_out);

/**
 * Calls keystore_secp256k1_sign by using the cached keypath and msg32 from
 * `keystore_nonce_secp256k1_commit`.
 * @return false if there is no cached data or keystore_secp256k1_sign() fails.
 */
USE_RESULT bool keystore_nonce_secp256k1_sign(
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out);

/**
 * Clears the signing data. Must be called at some point after `keystore_nonce_secp256k1_commit`,
 * either after `keystore_nonce_secp256k1_sign` or instead of it if it will never be called (signing
 * process aborted).
 */
void keystore_nonce_clear(void);

#endif
