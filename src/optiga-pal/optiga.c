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

#include "optiga-pal/optiga.h"
#include "hal_delay.h"
#include "hardfault.h"
#include "optiga_crypt.h"
#include "optiga_util.h"
#include "pal/pal_i2c.h"
#include "pal/pal_os_datastore.h"
#include "pal/pal_os_timer.h"
#include "securechip/securechip.h"
#include "util.h"
#include "rust/rust.h"

// Number of times the first kdf slot can be used.
// The maxmimum does not seem to be specified, so we use something a little below the endurance
// indication of 600000 updates. See Solution Reference Manual Figure 32.
#define MONOTONIC_COUNTER_MAX_USE (590000)

#define OPTIGA_DATA_OBJECT_ID_AES_SYMKEY 0xE200
#define OPTIGA_DATA_OBJECT_ID_HMAC 0xF1D0
#define OPTIGA_DATA_OBJECT_ID_ECC 0xE0F1
#define OPTIGA_DATA_OBJECT_ID_PLATFORM_BINDING 0xE140
#define OPTIGA_DATA_OBJECT_ID_COUNTER0 0xE120

#define ABORT_IF_NULL(ptr)           \
    do {                             \
        if ((ptr) == 0) {            \
            Abort("Not initalized"); \
        }                            \
    } while (0)

static optiga_util_t* util;
static optiga_crypt_t* crypt;

static const securechip_interface_functions_t* _ifs = NULL;

// The OPTIGA library is asynchronous and will schedule a callback when the command is done. The
// callback will set this shared variable to the result of the command.
static volatile optiga_lib_status_t optiga_lib_status;

static void optiga_lib_callback(void* callback_ctx, optiga_lib_status_t event)
{
    (void)callback_ctx;
    optiga_lib_status = event;
    printf("optiga_lib_callback 0x%.3x\n", event);
}

// Helper that is used in the main thread to busy wait for the callback to update the shared
// variable.
// It first checks the return status of the command, then busy waits, and then checks the
// asynchronous return status.
// Will return from caller if command failed.
// `return_status` will be updated with the actual return status
// Return statuses are documented in optiga_lib_return_codes.h
#define _WAIT(return_status, optiga_lib_status)                                              \
    do {                                                                                     \
        if ((return_status) != OPTIGA_UTIL_SUCCESS) {                                        \
            return (return_status);                                                          \
        }                                                                                    \
        while (OPTIGA_LIB_BUSY == (optiga_lib_status)) {                                     \
        }                                                                                    \
        if (OPTIGA_LIB_SUCCESS != (optiga_lib_status)) {                                     \
            return (optiga_lib_status);                                                      \
        }                                                                                    \
        (return_status) = (optiga_lib_status);                                               \
    } while (0)

// Value of Operational state
#define LCSO_STATE_CREATION (0x01)
// Value of Operational state
#define LCSO_STATE_OPERATIONAL (0x07)

// Currently set to Creation state(defualt value). At the real time/customer side this needs to be
// LCSO_STATE_OPERATIONAL (0x07)
#define FINAL_LCSO_STATE (LCSO_STATE_CREATION)

/* Platform Binding Shared Secret (0xE140) Metadata to be updated */
const uint8_t platform_binding_shared_secret_metadata_final[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    17,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0, 0x01, FINAL_LCSO_STATE,
    // Change/Write access. This allows updating the binding secret when LcsO < op.
    0xD0, 0x03, 0xE1, 0xFC, LCSO_STATE_OPERATIONAL,
    // Disallow reads
    0xD1, 0x01, 0xFF,
    // Allow execute
    0xD3, 0x01, 0x00,
    // Data object type set to PTFBIND (Platform binding secret)
    0xE8, 0x01, 0x22,
};

static const uint8_t e200_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    14,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0, 0x01, FINAL_LCSO_STATE,
    // Allow writes
    0xD0, 0x01, 0x00,
    // Disallow reads
    0xD1, 0x01, 0xFF,
    // Attach exeuction to counter at 0xE120
    0xD3, 0x03, 0x40, 0xE1, 0x20,
};

static const uint8_t ecc_metadata[] = {
    // Metadata tag in the data object
    0x20,
     // Number of bytes that follow
    15,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0, 0x01, FINAL_LCSO_STATE,
    // Key usage associated with key container: Sign (see Table 58 in Solution Reference Manual)
    0xE1, 0x01, 0x10,
    // Allow writes
    0xD0, 0x01, 0x00,
    // Disallow reads
    0xD1, 0x01, 0xFF,
    // Allow execution
    0xD3, 0x01, 0x00,
};

static const uint8_t hmac_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    15,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0, 0x01, FINAL_LCSO_STATE,
    // Data object type: PRESSEC (see Table 67 in Solution Reference Manual)
    0xE8, 0x01, 0x21,
    // Allow writes
    0xD0, 0x01, 0x00,
    // Disallow reads
    0xD1, 0x01, 0xFF,
    // Allow exe
    0xD3, 0x01, 0x00,
};

//
// Sync wrappers around optiga util/crypt functions
//

static optiga_lib_status_t _optiga_util_read_data_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint16_t offset,
    uint8_t* buffer,
    uint16_t* length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_read_data(me, optiga_oid, offset, buffer, length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_read_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t* buffer,
    uint16_t* length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_read_metadata(me, optiga_oid, buffer, length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_write_data_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t write_type,
    uint16_t offset,
    const uint8_t* buffer,
    uint16_t length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_util_write_data(me, optiga_oid, write_type, offset, buffer, length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_write_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    const uint8_t* buffer,
    uint8_t length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_write_metadata(me, optiga_oid, buffer, length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_open_application_sync(
    optiga_util_t* me,
    bool_t perform_restore)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_open_application(me, perform_restore);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_close_application_sync(
    optiga_util_t* me,
    bool_t perform_hibernate)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_close_application(me, perform_hibernate);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_hmac_sync(
    optiga_crypt_t* me,
    optiga_hmac_type_t type,
    uint16_t secret,
    const uint8_t* input_data,
    uint32_t input_data_length,
    uint8_t* mac,
    uint32_t* mac_length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_crypt_hmac(me, type, secret, input_data, input_data_length, mac, mac_length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_ecc_generate_keypair_sync(
    optiga_crypt_t *me,
    optiga_ecc_curve_t curve_id,
    uint8_t key_usage,
    bool_t export_private_key,
    void *private_key,
    uint8_t *public_key,
    uint16_t *public_key_length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_crypt_ecc_generate_keypair(me, curve_id, key_usage, export_private_key, private_key, public_key, public_key_length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_ecdsa_sign_sync(
    optiga_crypt_t *me,
    const uint8_t *digest,
    uint8_t digest_length,
    optiga_key_id_t private_key,
    uint8_t *signature,
    uint16_t *signature_length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_crypt_ecdsa_sign(me, digest, digest_length, private_key, signature, signature_length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_symmetric_encrypt_sync(
    optiga_crypt_t *me,
    optiga_symmetric_encryption_mode_t encryption_mode,
    optiga_key_id_t symmetric_key_oid,
    const uint8_t *plain_data,
    uint32_t plain_data_length,
    const uint8_t *iv,
    uint16_t iv_length,
    const uint8_t *associated_data,
    uint16_t associated_data_length,
    uint8_t *encrypted_data,
    uint32_t *encrypted_data_length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_crypt_symmetric_encrypt(me, encryption_mode, symmetric_key_oid, plain_data, plain_data_length,
                                       iv, iv_length, associated_data, associated_data_length, encrypted_data, encrypted_data_length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_hkdf_sync(
    optiga_crypt_t *me,
    optiga_hkdf_type_t type,
    uint16_t secret,
    const uint8_t *salt,
    uint16_t salt_length,
    const uint8_t *info,
    uint16_t info_length,
    uint16_t derived_key_length,
    bool_t export_to_host,
    uint8_t *derived_key)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_crypt_hkdf(me, type, secret, salt, salt_length, info, info_length, derived_key_length, export_to_host, derived_key);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_random_sync(
    optiga_crypt_t* me,
    optiga_rng_type_t rng_type,
    uint8_t* random_data,
    uint16_t random_data_length)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_random(me, rng_type, random_data, random_data_length);
    _WAIT(res, optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_symmetric_generate_key_sync(
    optiga_crypt_t *me,
    optiga_symmetric_key_type_t key_type,
    uint8_t key_usage,
    bool_t export_symmetric_key,
    void *symmetric_key)
{
    ABORT_IF_NULL(me);

    optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_symmetric_generate_key(me, key_type, key_usage, export_symmetric_key, symmetric_key);
    _WAIT(res, optiga_lib_status);
    return res;
}

bool optiga_monotonic_increments_remaining(uint32_t* remaining_out)
{
    uint8_t buf[4] = {0};
    uint16_t size = sizeof(buf);
    optiga_lib_status_t res = _optiga_util_read_data_sync(util, OPTIGA_DATA_OBJECT_ID_COUNTER0, 0, buf, &size);
    if (res != OPTIGA_LIB_SUCCESS) {
        return false;
    }

    uint32_t counter = optiga_common_get_uint32(buf);
    if (counter > MONOTONIC_COUNTER_MAX_USE) {
        Abort("optiga monotonic counter larget than max");
    }
    *remaining_out = MONOTONIC_COUNTER_MAX_USE - counter;
    return true;
}

bool optiga_update_keys(void)
{
    ABORT_IF_NULL(_ifs);
    ABORT_IF_NULL(_ifs->random_32_bytes);

    uint8_t new_key[32] = {0};
    _ifs->random_32_bytes(new_key);

    optiga_lib_status_t res = _optiga_util_write_data_sync(
               util,
               OPTIGA_DATA_OBJECT_ID_HMAC,
               OPTIGA_UTIL_ERASE_AND_WRITE,
               0x00,
               new_key,
               sizeof(new_key));
    if (res != OPTIGA_UTIL_SUCCESS) {
        return false;
    }

    optiga_key_id_t keyid = OPTIGA_KEY_ID_SECRET_BASED;
    res = _optiga_crypt_symmetric_generate_key_sync(
        crypt,
        OPTIGA_SYMMETRIC_AES_256,
        OPTIGA_KEY_USAGE_ENCRYPTION,
        false,
        &keyid);
    util_log("genkey: %x", res);

    return res == OPTIGA_UTIL_SUCCESS;
}

/* static void _experiment_locks(void) */
/* { */
/*     /\* const uint8_t metadata[] = { *\/ */
/*     /\*     0x20, *\/ */
/*     /\*     18, *\/ */
/*     /\*     // Data object type set to PRESSEC *\/ */
/*     /\*     0xE8, 0x01, 0x21, *\/ */
/*     /\*     0xD0, 0x03, 0xE1, 0xFC, LCSO_STATE_OPERATIONAL, // allow change LcsO < op *\/ */
/*     /\*     0xD1, 0x03, 0x70, 0xFC, LCSO_STATE_OPERATIONAL, // allow read LcsG < op *\/ */
/*     /\*     0xD3, 0x03, 0xE0, 0xFC, LCSO_STATE_OPERATIONAL, // allow exe LcsA < op *\/ */
/*     /\* }; *\/ */

/* #define OPTIGA_DATA_OBJECT_ID_EXPERIMENT 0xF1DB */
/*     optiga_lib_status_t res; */
/*     /\* res = _optiga_util_write_metadata_sync( *\/ */
/*     /\*     util, *\/ */
/*     /\*     OPTIGA_DATA_OBJECT_ID_EXPERIMENT, *\/ */
/*     /\*     metadata, *\/ */
/*     /\*     sizeof(metadata)); *\/ */
/*     /\* if (res != OPTIGA_LIB_SUCCESS) { *\/ */
/*     /\*     util_log("experiment metadata failed: %x", res); *\/ */
/*     /\*     return; *\/ */
/*     /\* } *\/ */
/*     const uint8_t metadata_lcso[] = { */
/*         0x20, */
/*         3, */
/*         0xC0, 0x01, 0x07, */
/*     }; */

/*     res = _optiga_util_write_metadata_sync( */
/*         util, */
/*         OPTIGA_DATA_OBJECT_ID_EXPERIMENT, */
/*         metadata_lcso, */
/*         sizeof(metadata_lcso)); */
/*     if (res != OPTIGA_LIB_SUCCESS) { */
/*         util_log("experiment set lcso failed: %x", res); */
/*         return; */
/*     } */


/*     uint8_t data[32] = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; */
/*     (void)_optiga_util_read_data_sync; */
/*     res = _optiga_util_write_data_sync( */
/*         util, */
/*         OPTIGA_DATA_OBJECT_ID_EXPERIMENT, */
/*         OPTIGA_UTIL_ERASE_AND_WRITE, */
/*         0, */
/*         data, */
/*         sizeof(data)); */
/*     if (res != OPTIGA_LIB_SUCCESS) { */
/*         util_log("experiment fail write data: %x", res); */
/*         return; */
/*     } */

/*     /\* uint8_t msg[32] = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"; *\/ */
/*     /\* uint8_t mac_out[32]; *\/ */
/*     /\* uint32_t mac_out_len = 32; *\/ */
/*     /\* res = _optiga_crypt_hmac_sync( *\/ */
/*     /\*     crypt, OPTIGA_HMAC_SHA_256, OPTIGA_DATA_OBJECT_ID_EXPERIMENT, msg, sizeof(msg), *\/ */
/*     /\*     mac_out, &mac_out_len); *\/ */
/*     /\* if (res != OPTIGA_LIB_SUCCESS) { *\/ */
/*     /\*     util_log("hmac experiemnt fail err=%x", res); *\/ */
/*     /\*     return; *\/ */
/*     /\* } *\/ */
/*     /\* util_log("hmac experiment: %d, %s", (int)mac_out_len, util_dbg_hex(mac_out, mac_out_len)); *\/ */


/*     /\* uint8_t data_lcs[1] = {0x01}; *\/ */
/*     /\* (void)_optiga_util_read_data_sync; *\/ */
/*     /\* res = _optiga_util_write_data_sync( *\/ */
/*     /\*     util, *\/ */
/*     /\*     0xF1C0, // LcsA *\/ */
/*     /\*     //0xE0C0, // LcsG *\/ */
/*     /\*     OPTIGA_UTIL_ERASE_AND_WRITE, *\/ */
/*     /\*     0, *\/ */
/*     /\*     data_lcs, *\/ */
/*     /\*     sizeof(data_lcs)); *\/ */
/*     /\* if (res != OPTIGA_LIB_SUCCESS) { *\/ */
/*     /\*     util_log("experiment fail LcsG: %x", res); *\/ */
/*     /\*     return; *\/ */
/*     /\* } *\/ */


/*     /\* memset(data, 0, sizeof(data)); *\/ */
/*     /\* uint16_t size = sizeof(data); *\/ */
/*     /\* res = _optiga_util_read_data_sync(util, OPTIGA_DATA_OBJECT_ID_EXPERIMENT, 0, data, &size); *\/ */
/*     /\* if (res != OPTIGA_LIB_SUCCESS) { *\/ */
/*     /\*     util_log("experiment fail read data: %x", res); *\/ */
/*     /\*     return; *\/ */
/*     /\* } *\/ */
/*     util_log("experiment ok"); */
/* } */

// Setup shielded communication.
// Writes the shared secret to the chip 0xE140 data object and sets the metadata.
// See solution reference manual 2.3.4 "Use case: Pair OPTIGA™ Trust M with host (pre-shared secret based)".
static bool _setup_shielded_communication(void)
{
    optiga_lib_status_t res;

    uint8_t current_metadata[1000] = {0};
    uint16_t current_metadata_size = sizeof(current_metadata);

    res = _optiga_util_read_metadata_sync(
            util,
            OPTIGA_DATA_OBJECT_ID_PLATFORM_BINDING,
            current_metadata,
            &current_metadata_size);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: read binding secret metadata: %x", res);
        return false;
    }
    util_log("current shared secret metadata: %s", util_dbg_hex(current_metadata, current_metadata_size));
    // Check that the LcsO metadata tag (0xC0 0x01 LCSO) is present, as we want to read the current
    // LcsO.
    if (current_metadata_size < 5 || current_metadata[0] != 0x20 || current_metadata[2] != 0xC0 || current_metadata[3] != 0x01) {
        util_log("unexpected shared secret metadata bytes");
        return false;
    }

    if (current_metadata[4] >= LCSO_STATE_OPERATIONAL) {
        util_log("shared secret already setup");
        return true;
    }

    uint8_t platform_binding_secret[32];
    uint16_t platform_binding_secret_size = sizeof(platform_binding_secret);

    pal_status_t pal_res = pal_os_datastore_read(
        OPTIGA_PLATFORM_BINDING_SHARED_SECRET_ID, platform_binding_secret, &platform_binding_secret_size);
    if (PAL_STATUS_SUCCESS != pal_res || platform_binding_secret_size != sizeof(platform_binding_secret)) {
        util_log("failed datastore read: %x", pal_res);
        return false;
    }

    // We write the binding secret before updating the metadata, as the metadata update locks the
    // slot.
    OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(util, OPTIGA_COMMS_NO_PROTECTION);
    res = _optiga_util_write_data_sync(
        util,
        OPTIGA_DATA_OBJECT_ID_PLATFORM_BINDING,
        OPTIGA_UTIL_ERASE_AND_WRITE,
        0,
        platform_binding_secret,
        sizeof(platform_binding_secret));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: write binding secret to chip: %x", res);
        return false;
    }

    OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(util, OPTIGA_COMMS_NO_PROTECTION);
    res = _optiga_util_write_metadata_sync(
        util,
        OPTIGA_DATA_OBJECT_ID_PLATFORM_BINDING,
        platform_binding_shared_secret_metadata_final,
        sizeof(platform_binding_shared_secret_metadata_final));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: write metadata of platform binding: %x", res);
        return false;
    }

    return true;
}

static bool _write_config(void)
{
    if (!_setup_shielded_communication()) {
        return false;
    }

    // Configure AES secret key
    optiga_lib_status_t res = _optiga_util_write_metadata_sync(
        util,
        OPTIGA_DATA_OBJECT_ID_AES_SYMKEY,
        e200_metadata,
        sizeof(e200_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("e200 metadata update failed: %x", res);
        return false;
    }


    //
    // Configure HMAC data object
    //
    rust_log("HMAC metadata");
    res = _optiga_util_write_metadata_sync(
        util, OPTIGA_DATA_OBJECT_ID_HMAC, hmac_metadata, sizeof(hmac_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("HMAC metadata failed: %x", res);
        return false;
    }

    // Configure the monotonic counter.
    // Table 73, "Counter".
    // Bytes 0-3 are the initial counter value, set to 0.
    // Bytes 4-7 are the threshold.
    // Ints are encoded as uint32 big endian.
    uint8_t counter_buf[8] = {0};
    optiga_common_set_uint32(&counter_buf[4], MONOTONIC_COUNTER_MAX_USE);
    res = _optiga_util_write_data_sync(
        util,
        OPTIGA_DATA_OBJECT_ID_COUNTER0,
        OPTIGA_UTIL_ERASE_AND_WRITE,
        0,
        counter_buf,
        sizeof(counter_buf));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: write initial counter data %x", res);
        return false;
    }

    // TODO write counter metadata

    // ECC slot metadata
    res = _optiga_util_write_metadata_sync(
        util,
        OPTIGA_DATA_OBJECT_ID_ECC,
        ecc_metadata,
        sizeof(ecc_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("ECC metadata update failed: %x", res);
        return false;
    }

    // TODO debug, remove.
    if(!optiga_update_keys()) {
        util_log("update hmac key failed");
        return false;
    }
    util_log("write config OK");

    /* _experiment_locks(); */
    return true;
}

static bool _factory_setup(void)
{
    optiga_lib_status_t res;

    util = optiga_util_create(OPTIGA_INSTANCE_ID_0, optiga_lib_callback, NULL);
    if (NULL == util) {
        util_log("couldn't create optiga util");
        return false;
    }

    crypt = optiga_crypt_create(OPTIGA_INSTANCE_ID_0, optiga_lib_callback, NULL);
    if (NULL == crypt) {
        util_log("couldn't create optiga crypt");
        return false;
    }

    OPTIGA_UTIL_SET_COMMS_PROTOCOL_VERSION(util, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);
    OPTIGA_CRYPT_SET_COMMS_PROTOCOL_VERSION(crypt, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);

    OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(util, OPTIGA_COMMS_NO_PROTECTION);
    res = _optiga_util_open_application_sync(util, 0);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("failed to open util application: %x", res);
        return false;
    }

    if (!_write_config()) {
        return false;
    }

    res = _optiga_util_close_application_sync(util, 0);
    if (res != OPTIGA_LIB_SUCCESS) {
        return false;
    }

    if (NULL != crypt) {
        optiga_crypt_destroy(crypt);
        crypt = NULL;
    }

    if (NULL != util) {
        optiga_util_destroy(util);
        util = NULL;
    }

    return true;
}

static bool _verify_config(void)
{
    optiga_lib_status_t res;
    util = optiga_util_create(OPTIGA_INSTANCE_ID_0, optiga_lib_callback, NULL);
    if (NULL == util) {
        return false;
    }

    crypt = optiga_crypt_create(OPTIGA_INSTANCE_ID_0, optiga_lib_callback, NULL);
    if (NULL == crypt) {
        return false;
    }

    OPTIGA_UTIL_SET_COMMS_PROTOCOL_VERSION(util, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);
    OPTIGA_CRYPT_SET_COMMS_PROTOCOL_VERSION(crypt, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);

    if (crypt->protection_level != OPTIGA_COMMS_FULL_PROTECTION) {
        util_log("crypt protection level expected to be FULL");
        return false;
    }
    if (util->protection_level != OPTIGA_COMMS_FULL_PROTECTION) {
        util_log("util protection level expected to be FULL");
        return false;
    }

    res = _optiga_util_open_application_sync(util, 0);
    if (res != OPTIGA_LIB_SUCCESS) {
        return false;
    }
    /* uint8_t buf[8] = {0}; */
    /* uint16_t size = sizeof(buf); */
    /* res = _optiga_util_read_data_sync(util, OPTIGA_DATA_OBJECT_ID_COUNTER0, 0, buf, &size); */
    /* if (res != OPTIGA_LIB_SUCCESS) { */
    /*     return false; */
    /* } */
    /* util_log("CTR %d %x %x %x %x %x %x %x %x", size, buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]); */

    /* uint8_t hmac[32] = {0}; */
    /* uint8_t msg[32] = "\x20\x5c\x85\x01\x9f\x41\xe8\x8c\x54\x46\x26\x57\x01\x09\x1e\x46\x36\xb4\x3e\xb3\x98\xee\xad\x61\xad\x0f\x8c\x58\x93\xcd\xd4\x60"; */
    /* if (securechip_kdf_rollkey(msg, 32, hmac) != 0) { */
    /*     Abort("kdf fail"); */
    /* } */
    /* util_log("hmac result: %s", util_dbg_hex(hmac, sizeof(hmac))); */

    /* size = sizeof(buf); */
    /* res = _optiga_util_read_data_sync(util, OPTIGA_DATA_OBJECT_ID_COUNTER0, 0, buf, &size); */
    /* if (res != OPTIGA_LIB_SUCCESS) { */
    /*     return false; */
    /* } */
    /* util_log("CTR %d %x %x %x %x %x %x %x %x", size, buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]); */


    return true;
}

int optiga_setup(const securechip_interface_functions_t* ifs)
{
    if (ifs == NULL) {
        return SC_ERR_IFS;
    }
    _ifs = ifs;

    util_log("optiga_setup");

    // A timer is used to provide the OPTIGA library with the ability to schedule work on the main
    // event loop
    pal_timer_init();

#if true // FACTORYSETUP == 1
    bool res = _factory_setup();
    if (!res) {
        return 1;
    }
#endif

    if (_verify_config()) {
        return 0;
    }
    return SC_ERR_INVALID_ARGS;
}

int optiga_kdf_internal(const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    ABORT_IF_NULL(crypt);
    optiga_lib_status_t res;

    uint8_t mac_out[16] = {0};
    uint32_t mac_out_len = sizeof(mac_out);

    res = _optiga_crypt_symmetric_encrypt_sync(
        crypt, OPTIGA_SYMMETRIC_CMAC, OPTIGA_DATA_OBJECT_ID_AES_SYMKEY, msg, len, NULL, 0, NULL, 0, mac_out, &mac_out_len);
    if (res != OPTIGA_LIB_SUCCESS || mac_out_len != sizeof(mac_out)) {
        util_log("cmac fail err=%x", res);
        return 1;
    }
    rust_sha256(mac_out, mac_out_len, kdf_out);

    return 0;
}

int optiga_kdf_external(const uint8_t* msg, size_t len, uint8_t* mac_out)
{
    ABORT_IF_NULL(crypt);
    optiga_lib_status_t res;
    // The equivalient of python `mac_out = hmac.new(key, msg[:len], hashlib.sha256).digest()`

    uint32_t mac_out_len = 32;

    (void)_optiga_crypt_hkdf_sync;

    res = _optiga_crypt_hmac_sync(
        crypt, OPTIGA_HMAC_SHA_256, OPTIGA_DATA_OBJECT_ID_HMAC, msg, len, mac_out, &mac_out_len);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("kdf fail err=%x", res);
        return 1;
    }
    if (mac_out_len != 32) {
        return 1;
    }

    return 0;
}

bool optiga_gen_attestation_key(uint8_t* pubkey_out)
{
    ABORT_IF_NULL(crypt);
    optiga_key_id_t slot = OPTIGA_KEY_ID_E0F1;
    uint8_t pubkey_der[68] = {0};
    uint16_t pubkey_der_size = sizeof(pubkey_der);
    optiga_lib_status_t res = _optiga_crypt_ecc_generate_keypair_sync(
        crypt,
        OPTIGA_ECC_CURVE_NIST_P_256,
        OPTIGA_KEY_USAGE_SIGN,
        false,
        (void*)&slot,
        pubkey_der,
        &pubkey_der_size);
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("gen keypair failed: %x", res);
        return false;
    }
    // Parse DER "BIT STRING", see Solution Reference Manual 6.2.2,
    // example for ECC NIST-P256.
    // The 64 byte X/Y values are the last 64 bytes.
    if (pubkey_der_size != 68 || !MEMEQ(pubkey_der, "\x03\x42\x00\x04", 4)) {
        return false;
    }
    memcpy(pubkey_out, pubkey_der + 4, 64);
    return true;
}

bool optiga_attestation_sign(const uint8_t* challenge, uint8_t* signature_out)
{
    ABORT_IF_NULL(crypt);
    uint8_t sig_der[70] = {0};
    uint16_t sig_der_size = sizeof(sig_der);
    optiga_lib_status_t res = _optiga_crypt_ecdsa_sign_sync(
        crypt,
        challenge,
        32,
        OPTIGA_KEY_ID_E0F1,
        sig_der,
        &sig_der_size);
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("sign failed: %x", res);
        return false;
    }
    // Parse signature, see Solution Reference Manual 6.2.2,
    // example for ECC NIST-P256 signature.
    // The R/S components are
    util_log("sign %d", sig_der_size);
    return rust_der_parse_optiga_signature(
            rust_util_bytes(sig_der, sig_der_size),
            rust_util_bytes_mut(signature_out, 64));
}

// rand_out must be 32 bytes
bool optiga_random(uint8_t* rand_out)
{
    optiga_lib_status_t res = _optiga_crypt_random_sync(crypt, OPTIGA_RNG_TYPE_TRNG, rand_out, 32);
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("optiga_random failed: %x", res);
        return false;
    }
    return true;
}

bool optiga_model(securechip_model_t* model_out)
{
    *model_out = OPTIGA_TRUST_M_V3;
    return true;
}
