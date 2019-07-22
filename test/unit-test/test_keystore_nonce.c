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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <keystore/keystore_nonce.h>

#include <wally_bip32.h>
#include <wally_crypto.h>

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-prototypes"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wbad-function-cast"

bool __wrap_keystore_secp256k1_sign(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out)
{
    check_expected(keypath);
    check_expected(keypath_len);
    check_expected(msg32);
    check_expected(host_nonce32);
    check_expected(sig_compact_out);
    check_expected(recid_out);
    return true;
}

bool __wrap_keystore_secp256k1_nonce_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* client_commitment_out)
{
    check_expected(keypath);
    check_expected(keypath_len);
    check_expected(msg32);
    check_expected(host_commitment);
    check_expected(client_commitment_out);
    return true;
}

static void _test_keystore_nonce(void** state)
{
    uint32_t keypath[] = {
        44 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0 + BIP32_INITIAL_HARDENED_CHILD,
        0,
        0,
    };

    uint8_t msg[32];
    memset(msg, 0x23, sizeof(msg));

    uint8_t host_nonce[32];
    memset(host_nonce, 0x55, sizeof(host_nonce));

    uint8_t client_commitment[32] = {0};

    uint8_t sig[64];
    int recid;

    // No cached data yet.
    assert_false(keystore_nonce_secp256k1_sign(host_nonce, sig, &recid));

    for (int i = 0; i < 3; i++) {
        // Modify params to check that the right thing is cached.
        keypath[4] = i;
        msg[0] = i;
        host_nonce[0] = i;
        uint8_t host_nonce_commitment[32];
        wally_sha256(host_nonce, sizeof(host_nonce), host_nonce_commitment, 32);

        { // commit
            expect_memory(
                __wrap_keystore_secp256k1_nonce_commit, keypath, keypath, sizeof(keypath));
            expect_value(__wrap_keystore_secp256k1_nonce_commit, keypath_len, 5);
            expect_memory(__wrap_keystore_secp256k1_nonce_commit, msg32, msg, sizeof(msg));
            expect_memory(
                __wrap_keystore_secp256k1_nonce_commit,
                host_commitment,
                host_nonce_commitment,
                sizeof(host_nonce_commitment));
            expect_value(
                __wrap_keystore_secp256k1_nonce_commit, client_commitment_out, client_commitment);
            assert_true(keystore_nonce_secp256k1_commit(
                keypath, 5, msg, host_nonce_commitment, client_commitment));

            // Can't commit again, already has cached data
            assert_false(keystore_nonce_secp256k1_commit(
                keypath, 5, msg, host_nonce_commitment, client_commitment));
        }
        { // sign
            expect_memory(__wrap_keystore_secp256k1_sign, keypath, keypath, sizeof(keypath));
            expect_value(__wrap_keystore_secp256k1_sign, keypath_len, 5);
            expect_memory(__wrap_keystore_secp256k1_sign, msg32, msg, sizeof(msg));
            expect_memory(
                __wrap_keystore_secp256k1_sign, host_nonce32, host_nonce, sizeof(host_nonce));
            expect_value(__wrap_keystore_secp256k1_sign, sig_compact_out, sig);
            expect_value(__wrap_keystore_secp256k1_sign, recid_out, &recid);
            assert_true(keystore_nonce_secp256k1_sign(host_nonce, sig, &recid));
        }
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_keystore_nonce),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}

#pragma GCC diagnostic pop
