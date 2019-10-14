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

#include <keystore.h>
#include <secp256k1_ecdsa_sign_to_contract.h>
#include <secp256k1_recovery.h>
#include <securechip/securechip.h>
#include <util.h>

#include <stdint.h>
#include <stdio.h>
#include <string.h>

static uint8_t _mock_seed[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

static uint8_t _mock_seed_2[32] = {
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
};

static uint8_t _mock_bip39_seed[64] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

static const uint32_t _keypath[] = {
    44 + BIP32_INITIAL_HARDENED_CHILD,
    0 + BIP32_INITIAL_HARDENED_CHILD,
    0 + BIP32_INITIAL_HARDENED_CHILD,
    0,
    5,
};
// seckey at the above keypath with the above bip39 seed.
static const uint8_t _expected_seckey[32] = {
    0x4e, 0x64, 0xdf, 0xd3, 0x3a, 0xae, 0x66, 0xc4, 0xc7, 0x52, 0x6c, 0xf0, 0x2e, 0xe8, 0xae, 0x3f,
    0x58, 0x92, 0x32, 0x9d, 0x67, 0xdf, 0xd4, 0xad, 0x05, 0xe9, 0xc3, 0xd0, 0x6e, 0xdf, 0x74, 0xfb,
};

static uint8_t _password_salted_hashed_stretch_in[32] = {
    0x5e, 0x88, 0x48, 0x98, 0xda, 0x28, 0x04, 0x71, 0x51, 0xd0, 0xe5, 0x6f, 0x8d, 0xc6, 0x29, 0x27,
    0x73, 0x60, 0x3d, 0x0d, 0x6a, 0xab, 0xbd, 0xd6, 0x2a, 0x11, 0xef, 0x72, 0x1d, 0x15, 0x42, 0xd8,
};

static uint8_t _password_salted_hashed_stretch_out[32] = {
    0x73, 0x60, 0x3d, 0x0d, 0x6a, 0xab, 0xbd, 0xd6, 0x2a, 0x11, 0xef, 0x72, 0x1d, 0x15, 0x42, 0xd8,
    0x5e, 0x88, 0x48, 0x98, 0xda, 0x28, 0x04, 0x71, 0x51, 0xd0, 0xe5, 0x6f, 0x8d, 0xc6, 0x29, 0x27,
};

static uint8_t _password_salted_hashed_stretch_out_invalid[32] = {
    0x72, 0x60, 0x3d, 0x0d, 0x6a, 0xab, 0xbd, 0xd6, 0x2a, 0x11, 0xef, 0x72, 0x1d, 0x15, 0x42, 0xd8,
    0x5e, 0x88, 0x48, 0x98, 0xda, 0x28, 0x04, 0x71, 0x51, 0xd0, 0xe5, 0x6f, 0x8d, 0xc6, 0x29, 0x27,
};

static uint8_t _kdf_out_1[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
};

static uint8_t _kdf_out_2[32] = {
    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
};

static uint8_t _kdf_out_3[32] = {
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
};

int __real_secp256k1_ecdsa_s2c_sign(
    const secp256k1_context* ctx,
    secp256k1_ecdsa_signature* sig,
    secp256k1_s2c_opening* s2c_opening,
    const unsigned char* msg32,
    const unsigned char* seckey,
    const unsigned char* s2c_data32,
    int* recid);

static const unsigned char* _sign_expected_msg = NULL;
static const unsigned char* _sign_expected_seckey = NULL;
int __wrap_secp256k1_ecdsa_s2c_sign(
    const secp256k1_context* ctx,
    secp256k1_ecdsa_signature* sig,
    secp256k1_s2c_opening* s2c_opening,
    const unsigned char* msg32,
    const unsigned char* seckey,
    const unsigned char* s2c_data32,
    int* recid)
{
    if (_sign_expected_msg != NULL) {
        assert_memory_equal(_sign_expected_msg, msg32, 32);
        _sign_expected_msg = NULL;
    }
    if (_sign_expected_seckey != NULL) {
        assert_memory_equal(_sign_expected_seckey, seckey, 32);
        _sign_expected_seckey = NULL;
    }
    return __real_secp256k1_ecdsa_s2c_sign(ctx, sig, s2c_opening, msg32, seckey, s2c_data32, recid);
}

bool __wrap_salt_hash_data(
    const uint8_t* data,
    size_t data_len,
    const char* purpose,
    uint8_t* hash_out)
{
    check_expected(purpose);
    memcpy(hash_out, (const void*)mock(), 32);
    return true;
}

bool __real_cipher_aes_hmac_encrypt(
    const unsigned char* in,
    int in_len,
    uint8_t* out,
    int* out_len,
    const uint8_t* secret);

bool __wrap_cipher_aes_hmac_encrypt(
    const unsigned char* in,
    int in_len,
    uint8_t* out,
    int* out_len,
    const uint8_t* secret)
{
    check_expected(secret);
    return __real_cipher_aes_hmac_encrypt(in, in_len, out, out_len, secret);
}

static bool _reset_reset_called = false;
void __wrap_reset_reset(void)
{
    _reset_reset_called = true;
}

static bool _pubkeys_equal(
    const secp256k1_context* ctx,
    const secp256k1_pubkey* pubkey1,
    const secp256k1_pubkey* pubkey2)
{
    uint8_t pubkey1_bytes[33];
    uint8_t pubkey2_bytes[33];
    size_t len = 33;
    assert_true(
        secp256k1_ec_pubkey_serialize(ctx, pubkey1_bytes, &len, pubkey1, SECP256K1_EC_COMPRESSED));
    assert_true(
        secp256k1_ec_pubkey_serialize(ctx, pubkey2_bytes, &len, pubkey2, SECP256K1_EC_COMPRESSED));
    return memcmp(pubkey1_bytes, pubkey2_bytes, len) == 0;
}

static void _test_keystore_get_xpub(void** state)
{
    secp256k1_context* ctx = wally_get_secp_context();

    struct ext_key xpub = {0};

    mock_state(NULL, NULL);
    // fails because keystore is locked
    assert_false(keystore_get_xpub(_keypath, sizeof(_keypath) / sizeof(uint32_t), &xpub));

    mock_state(_mock_seed, _mock_bip39_seed);
    assert_true(keystore_get_xpub(_keypath, sizeof(_keypath) / sizeof(uint32_t), &xpub));

    secp256k1_pubkey expected_pubkey;
    assert_true(secp256k1_ec_pubkey_create(ctx, &expected_pubkey, _expected_seckey));

    secp256k1_pubkey pubkey;
    assert_true(secp256k1_ec_pubkey_parse(ctx, &pubkey, xpub.pub_key, sizeof(xpub.pub_key)));

    assert_true(_pubkeys_equal(ctx, &pubkey, &expected_pubkey));

    char* xpub_string;
    // Make sure it's a public key, no
    assert_false(bip32_key_to_base58(&xpub, BIP32_FLAG_KEY_PRIVATE, &xpub_string) == WALLY_OK);
    assert_true(bip32_key_to_base58(&xpub, BIP32_FLAG_KEY_PUBLIC, &xpub_string) == WALLY_OK);
    assert_string_equal(
        xpub_string,
        "xpub6Gmp9vKrJrVbU5JDcPRm6UmJPjTBurWfqow6w3BoK46E6mVyScMfTXd66WFeLfRa7Ug4iGMWDpWLpZAYcuUHyz"
        "cWZCqh8393rbuMoerRK1p");
    wally_free_string(xpub_string);
}

static void _test_keystore_secp256k1_sign(void** state)
{
    secp256k1_context* ctx = wally_get_secp_context();

    secp256k1_pubkey expected_pubkey;
    assert_true(secp256k1_ec_pubkey_create(ctx, &expected_pubkey, _expected_seckey));

    uint8_t msg[32] = {0};
    memset(msg, 0x88, sizeof(msg));
    uint8_t sig[64] = {0};

    // uint8_t host_nonce[32] = {0};
    uint8_t* host_nonce = NULL;
    {
        mock_state(NULL, NULL);
        // fails because keystore is locked
        assert_false(keystore_secp256k1_sign(
            _keypath, sizeof(_keypath) / sizeof(uint32_t), msg, host_nonce, sig, NULL));
    }
    {
        mock_state(_mock_seed, _mock_bip39_seed);

        _sign_expected_seckey = _expected_seckey;
        _sign_expected_msg = msg;
        // check sig by verifying it against the msg.
        assert_true(keystore_secp256k1_sign(
            _keypath, sizeof(_keypath) / sizeof(uint32_t), msg, host_nonce, sig, NULL));
        secp256k1_ecdsa_signature secp256k1_sig = {0};
        assert_true(secp256k1_ecdsa_signature_parse_compact(ctx, &secp256k1_sig, sig));
        assert_true(secp256k1_ecdsa_verify(ctx, &secp256k1_sig, msg, &expected_pubkey));
    }
    { // test recoverable id (recid)
        int recid;
        assert_true(keystore_secp256k1_sign(
            _keypath, sizeof(_keypath) / sizeof(uint32_t), msg, host_nonce, sig, &recid));
        assert_int_equal(recid, 1);

        // Test recid by recovering the public key from the signature and checking against the
        // expected puklic key.
        secp256k1_ecdsa_recoverable_signature recoverable_sig;
        assert_true(
            secp256k1_ecdsa_recoverable_signature_parse_compact(ctx, &recoverable_sig, sig, recid));

        secp256k1_pubkey recovered_pubkey;
        assert_true(secp256k1_ecdsa_recover(ctx, &recovered_pubkey, &recoverable_sig, msg));

        assert_true(_pubkeys_equal(ctx, &recovered_pubkey, &expected_pubkey));
    }
}

static void _expect_stretch(bool valid)
{
    expect_string(__wrap_salt_hash_data, purpose, "keystore_seed_access_in");
    will_return(__wrap_salt_hash_data, _password_salted_hashed_stretch_in);

    // KDF 1
    expect_value(securechip_kdf, slot, SECURECHIP_SLOT_ROLLKEY);
    expect_memory(securechip_kdf, msg, _password_salted_hashed_stretch_in, 32);
    will_return(securechip_kdf, _kdf_out_1);

    // KDF 2
    expect_value(securechip_kdf, slot, SECURECHIP_SLOT_KDF);
    expect_memory(securechip_kdf, msg, _kdf_out_1, 32);
    will_return(securechip_kdf, _kdf_out_2);

    // KDF 3
    expect_value(securechip_kdf, slot, SECURECHIP_SLOT_KDF);
    expect_memory(securechip_kdf, msg, _kdf_out_2, 32);
    will_return(securechip_kdf, _kdf_out_3);

    expect_string(__wrap_salt_hash_data, purpose, "keystore_seed_access_out");
    will_return(
        __wrap_salt_hash_data,
        valid ? _password_salted_hashed_stretch_out : _password_salted_hashed_stretch_out_invalid);
}

static void _expect_encrypt_and_store_seed(void)
{
    will_return(__wrap_memory_is_initialized, false);

    _expect_stretch(true); // first stretch to encrypt
    _expect_stretch(true); // second stretch to verify

    // Fixture: hmac.new(_password_salted_hashed_stretch_out, _kdf_out_3,
    // hashlib.sha256).hexdigest()
    static uint8_t expected_secret[32] = {
        0x39, 0xa7, 0x4f, 0x75, 0xb6, 0x9d, 0x6c, 0x84, 0x5e, 0x18, 0x91,
        0x5b, 0xae, 0x29, 0xd1, 0x06, 0x12, 0x12, 0x40, 0x37, 0x7a, 0x79,
        0x97, 0x55, 0xd7, 0xcc, 0xe9, 0x26, 0x1e, 0x16, 0x91, 0x71,
    };
    expect_memory(__wrap_cipher_aes_hmac_encrypt, secret, expected_secret, 32);
}

static void _test_keystore_encrypt_and_store_seed(void** state)
{
    const char* password = "password";
    _expect_encrypt_and_store_seed();
    assert_true(keystore_encrypt_and_store_seed(_mock_seed, 32, password));
}

static void _expect_seeded(bool seeded)
{
    uint8_t seed[KEYSTORE_MAX_SEED_LENGTH];
    uint32_t len;
    assert_int_equal(seeded, keystore_copy_seed(seed, &len));
}

static void _test_keystore_unlock(void** state)
{
    mock_state(NULL, NULL); // reset to locked

    const char* password = "password";
    const uint8_t max_attempts = 10;
    uint8_t remaining_attempts;

    will_return(__wrap_memory_is_seeded, false);
    assert_int_equal(KEYSTORE_ERR_GENERIC, keystore_unlock(password, &remaining_attempts));
    _expect_encrypt_and_store_seed();
    assert_true(keystore_encrypt_and_store_seed(_mock_seed, 32, password));
    _expect_seeded(false);
    // Loop to check that unlocking unlocked works while unlocked.
    for (int i = 0; i < 3; i++) {
        _reset_reset_called = false;
        will_return(__wrap_memory_is_seeded, true);
        _expect_stretch(true);
        assert_int_equal(KEYSTORE_OK, keystore_unlock(password, &remaining_attempts));
        assert_int_equal(remaining_attempts, max_attempts);
        assert_false(_reset_reset_called);
        _expect_seeded(true);
    }

    { // Test that unlocking the keystore fails if it is already unlocked and the seed changed.
        // a) store different seed
        _reset_reset_called = false;
        _expect_encrypt_and_store_seed();
        assert_true(keystore_encrypt_and_store_seed(_mock_seed_2, 32, password));
        // b) fail to unlock (despite a correct password)
        will_return(__wrap_memory_is_seeded, true);
        _expect_stretch(true);
        assert_int_equal(KEYSTORE_ERR_GENERIC, keystore_unlock(password, &remaining_attempts));
        assert_false(_reset_reset_called);
    }

    // Invalid passwords until we run out of attempts.
    for (int i = 1; i <= max_attempts; i++) {
        _reset_reset_called = false;
        will_return(__wrap_memory_is_seeded, true);
        _expect_stretch(false);
        assert_int_equal(
            i >= max_attempts ? KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED
                              : KEYSTORE_ERR_INCORRECT_PASSWORD,
            keystore_unlock(password, &remaining_attempts));
        assert_int_equal(remaining_attempts, max_attempts - i);
        // Wrong password does not lock the keystore again if already unlocked.
        _expect_seeded(true);
        // reset_reset() called in last attempt
        assert_int_equal(i == max_attempts, _reset_reset_called);
    }

    // Trying again after max attempts is blocked immediately.
    _reset_reset_called = false;
    will_return(__wrap_memory_is_seeded, true);
    assert_int_equal(
        KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED, keystore_unlock(password, &remaining_attempts));
    assert_int_equal(remaining_attempts, 0);
    assert_true(_reset_reset_called);
}

static void _test_keystore_lock(void** state)
{
    mock_state(NULL, NULL);
    assert_true(keystore_is_locked());
    mock_state(_mock_seed, NULL);
    assert_true(keystore_is_locked());
    mock_state(_mock_seed, _mock_bip39_seed);
    assert_false(keystore_is_locked());
    keystore_lock();
    assert_true(keystore_is_locked());
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_keystore_get_xpub),
        cmocka_unit_test(_test_keystore_secp256k1_sign),
        cmocka_unit_test(_test_keystore_encrypt_and_store_seed),
        cmocka_unit_test(_test_keystore_unlock),
        cmocka_unit_test(_test_keystore_lock),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
