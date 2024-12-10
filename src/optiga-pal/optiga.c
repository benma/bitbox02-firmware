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
#include "rust/rust.h"
#include "securechip/securechip.h"
#include "util.h"

// Number of times the first kdf slot can be used.
// The maxmimum does not seem to be specified, so we use something a little below the endurance
// indication of 600000 updates. See Solution Reference Manual Figure 32.
#define MONOTONIC_COUNTER_MAX_USE (590000)

#define OPTIGA_DATA_OBJECT_ID_AES_SYMKEY 0xE200
#define OPTIGA_DATA_OBJECT_ID_HMAC 0xF1D0
#define OPTIGA_DATA_OBJECT_ID_ARBITRARY_DATA 0xF1D1
#define OPTIGA_DATA_OBJECT_ID_ATTESTATION 0xE0F1
#define OPTIGA_DATA_OBJECT_ID_PLATFORM_BINDING 0xE140
#define OPTIGA_DATA_OBJECT_ID_COUNTER0 0xE120

// See Solution Reference Manual Table "Data structure arbitrary data object".
#define ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE 140

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpacked"
#pragma GCC diagnostic ignored "-Wattributes"
typedef union {
    struct __attribute__((__packed__)) {
        uint32_t u2f_counter;
    } fields;
    uint8_t bytes[ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE];
} arbitrary_data_t;
#pragma GCC diagnostic pop

#define ABORT_IF_NULL(ptr)                    \
    do {                                      \
        if ((ptr) == 0) {                     \
            AbortAutoenter("Not initalized"); \
        }                                     \
    } while (0)

static optiga_util_t* _util;
static optiga_crypt_t* _crypt;

static const securechip_interface_functions_t* _ifs = NULL;

// The OPTIGA library is asynchronous and will schedule a callback when the command is done. The
// callback will set this shared variable to the result of the command.
static volatile optiga_lib_status_t _optiga_lib_status;

static void _optiga_lib_callback(void* callback_ctx, optiga_lib_status_t event)
{
    (void)callback_ctx;
    _optiga_lib_status = event;
}

// Helper that is used in the main thread to busy wait for the callback to update the shared
// variable.
// It first checks the return status of the command, then busy waits, and then checks the
// asynchronous return status.
// Will return from caller if command failed.
// `return_status` will be updated with the actual return status
// Return statuses are documented in optiga_lib_return_codes.h
#define _WAIT(return_status, optiga_lib_status)          \
    do {                                                 \
        if ((return_status) != OPTIGA_UTIL_SUCCESS) {    \
            return (return_status);                      \
        }                                                \
        while (OPTIGA_LIB_BUSY == (optiga_lib_status)) { \
        }                                                \
        if (OPTIGA_LIB_SUCCESS != (optiga_lib_status)) { \
            return (optiga_lib_status);                  \
        }                                                \
        (return_status) = (optiga_lib_status);           \
    } while (0)

// Value of Operational state
#define LCSO_STATE_CREATION (0x01)
// Value of Operational state
#define LCSO_STATE_OPERATIONAL (0x07)

// Currently set to Creation state(defualt value). At the real time/customer side this needs to be
// LCSO_STATE_OPERATIONAL (0x07)
#define FINAL_LCSO_STATE (LCSO_STATE_CREATION)

/* Platform Binding Shared Secret (0xE140) Metadata to be updated */
const uint8_t platform_binding_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    17,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Change/Write access. This allows updating the binding secret when LcsO < op.
    0xD0,
    0x03,
    0xE1,
    0xFC,
    LCSO_STATE_OPERATIONAL,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Allow execute
    0xD3,
    0x01,
    0x00,
    // Data object type set to PTFBIND (Platform binding secret)
    0xE8,
    0x01,
    0x22,
};

static const uint8_t aes_symkey_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    18,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Allow writes - GenSymkey requires this to be able to write.
    // However, writes from the host are forbidden.
    // Table 69 - 0xE200:
    // "The GetDataObject, and SetDataObject commands are not allowed for the data part of the key
    // object even if the metadata state the access rights differently"
    0xD0,
    0x01,
    0x00,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Attach execution to counter at 0xE120 and enforce shielded connection.
    // See Table 66 "Conf".
    0xD3,
    0x07,
    0x40,
    0xE1,
    0x20,
    // &&
    0xFD,
    // Enforce shielded connection. According to 4.4.1.7 "EncryptSym" shielded protection is
    // enforced anyway, but better to be explicit.
    0x20,
    0xE1,
    0x40,
};

static const uint8_t attestation_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    17,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Key usage associated with key container: Sign (see Table 58 in Solution Reference Manual)
    0xE1,
    0x01,
    0x10,
    // Allow writes - GenKeyPair requires this to be able to write.
    // However, writes from the host are forbidden.
    // Table 69 - 0xE2F1:
    // "The GetDataObject, and SetDataObject commands are not allowed for the data part of the key
    // object even if the metadata state the access rights differently"
    0xD0,
    0x01,
    0x00,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Execute: enforce shielded connection.
    // See Table 66 "Conf".
    0xD3,
    0x03,
    0x20,
    0xE1,
    0x40,
};

static const uint8_t hmac_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    19,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Data object type: PRESSEC (see Table 67 in Solution Reference Manual)
    0xE8,
    0x01,
    0x21,
    // Allow writes, enforce shielded connection.
    0xD0,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Execute: enforce shielded connection.
    // See Table 66 "Conf".
    0xD3,
    0x03,
    0x20,
    0xE1,
    0x40,
};

static const uint8_t arbitrary_data_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    19,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Data object type: BSTR (see Table 67 in Solution Reference Manual)
    0xE8,
    0x01,
    0x00,
    // Allow writes, enforce shielded connection.
    0xD0,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Allow reads, enforce shielded connection.
    0xD1,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Disallow exe
    0xD3,
    0x01,
    0xFF,
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

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_read_data(me, optiga_oid, offset, buffer, length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_read_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t* buffer,
    uint16_t* length)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_read_metadata(me, optiga_oid, buffer, length);
    _WAIT(res, _optiga_lib_status);
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

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_util_write_data(me, optiga_oid, write_type, offset, buffer, length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_write_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    const uint8_t* buffer,
    uint8_t length)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_write_metadata(me, optiga_oid, buffer, length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_open_application_sync(
    optiga_util_t* me,
    bool_t perform_restore)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_open_application(me, perform_restore);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_util_close_application_sync(
    optiga_util_t* me,
    bool_t perform_hibernate)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_close_application(me, perform_hibernate);
    _WAIT(res, _optiga_lib_status);
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

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_crypt_hmac(me, type, secret, input_data, input_data_length, mac, mac_length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_ecc_generate_keypair_sync(
    optiga_crypt_t* me,
    optiga_ecc_curve_t curve_id,
    uint8_t key_usage,
    bool_t export_private_key,
    void* private_key,
    uint8_t* public_key,
    uint16_t* public_key_length)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_ecc_generate_keypair(
        me, curve_id, key_usage, export_private_key, private_key, public_key, public_key_length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_ecdsa_sign_sync(
    optiga_crypt_t* me,
    const uint8_t* digest,
    uint8_t digest_length,
    optiga_key_id_t private_key,
    uint8_t* signature,
    uint16_t* signature_length)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_ecdsa_sign(
        me, digest, digest_length, private_key, signature, signature_length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_symmetric_encrypt_sync(
    optiga_crypt_t* me,
    optiga_symmetric_encryption_mode_t encryption_mode,
    optiga_key_id_t symmetric_key_oid,
    const uint8_t* plain_data,
    uint32_t plain_data_length,
    const uint8_t* iv,
    uint16_t iv_length,
    const uint8_t* associated_data,
    uint16_t associated_data_length,
    uint8_t* encrypted_data,
    uint32_t* encrypted_data_length)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_symmetric_encrypt(
        me,
        encryption_mode,
        symmetric_key_oid,
        plain_data,
        plain_data_length,
        iv,
        iv_length,
        associated_data,
        associated_data_length,
        encrypted_data,
        encrypted_data_length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_random_sync(
    optiga_crypt_t* me,
    optiga_rng_type_t rng_type,
    uint8_t* random_data,
    uint16_t random_data_length)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_random(me, rng_type, random_data, random_data_length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

static optiga_lib_status_t _optiga_crypt_symmetric_generate_key_sync(
    optiga_crypt_t* me,
    optiga_symmetric_key_type_t key_type,
    uint8_t key_usage,
    bool_t export_symmetric_key,
    void* symmetric_key)
{
    ABORT_IF_NULL(me);

    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_symmetric_generate_key(
        me, key_type, key_usage, export_symmetric_key, symmetric_key);
    _WAIT(res, _optiga_lib_status);
    return res;
}

// Setup shielded communication.
// Writes the shared secret to the chip 0xE140 data object and sets the metadata.
// See solution reference manual 2.3.4 "Use case: Pair OPTIGA™ Trust M with host (pre-shared secret
// based)".
static int _setup_shielded_communication(void)
{
    optiga_lib_status_t res;

    uint8_t current_metadata[1000] = {0};
    uint16_t current_metadata_size = sizeof(current_metadata);

    res = _optiga_util_read_metadata_sync(
        _util, OPTIGA_DATA_OBJECT_ID_PLATFORM_BINDING, current_metadata, &current_metadata_size);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: read binding secret metadata: %x", res);
        return res;
    }
    util_log(
        "current shared secret metadata: %s",
        util_dbg_hex(current_metadata, current_metadata_size));
    // Check that the LcsO metadata tag (0xC0 0x01 LCSO) is present, as we want to read the current
    // LcsO.
    if (current_metadata_size < 5 || current_metadata[0] != 0x20 || current_metadata[2] != 0xC0 ||
        current_metadata[3] != 0x01) {
        util_log("unexpected shared secret metadata bytes");
        return OPTIGA_ERR_UNEXPECTED_METADATA;
    }

    if (current_metadata[4] >= LCSO_STATE_OPERATIONAL) {
        util_log("shared secret already setup");
        return 0;
    }

    uint8_t platform_binding_secret[32];
    uint16_t platform_binding_secret_size = sizeof(platform_binding_secret);

    pal_status_t pal_res = pal_os_datastore_read(
        OPTIGA_PLATFORM_BINDING_SHARED_SECRET_ID,
        platform_binding_secret,
        &platform_binding_secret_size);
    if (PAL_STATUS_SUCCESS != pal_res ||
        platform_binding_secret_size != sizeof(platform_binding_secret)) {
        util_log("failed datastore read: %x", pal_res);
        return OPTIGA_ERR_PAL;
    }

    // We write the binding secret before updating the metadata, as the metadata update locks the
    // slot.
    res = _optiga_util_write_data_sync(
        _util,
        OPTIGA_DATA_OBJECT_ID_PLATFORM_BINDING,
        OPTIGA_UTIL_ERASE_AND_WRITE,
        0,
        platform_binding_secret,
        sizeof(platform_binding_secret));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: write binding secret to chip: %x", res);
        return res;
    }

    OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(_util, OPTIGA_COMMS_NO_PROTECTION);
    res = _optiga_util_write_metadata_sync(
        _util,
        OPTIGA_DATA_OBJECT_ID_PLATFORM_BINDING,
        platform_binding_metadata,
        sizeof(platform_binding_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: write metadata of platform binding: %x", res);
        return res;
    }

    return 0;
}

#if APP_U2F == 1 || FACTORYSETUP == 1
static bool _read_arbitrary_data(arbitrary_data_t* data_out)
{
    memset(data_out->bytes, 0x00, sizeof(data_out->bytes));
    uint16_t len = sizeof(data_out->bytes);
    optiga_lib_status_t res = _optiga_util_read_data_sync(
        _util, OPTIGA_DATA_OBJECT_ID_ARBITRARY_DATA, 0, data_out->bytes, &len);
    if (res != OPTIGA_UTIL_SUCCESS) {
        util_log("could not read arbitrary data: %x", res);
        return false;
    }
    if (len != sizeof(data_out->bytes)) {
        util_log(
            "arbitary data: expected to read size %d, but read %d. Data read: %s",
            (int)sizeof(data_out->bytes),
            (int)len,
            util_dbg_hex(data_out->bytes, len));
        return false;
    }
    return true;
}
#endif

static int _write_arbitrary_data(const arbitrary_data_t* data)
{
    optiga_lib_status_t res = _optiga_util_write_data_sync(
        _util,
        OPTIGA_DATA_OBJECT_ID_ARBITRARY_DATA,
        OPTIGA_UTIL_ERASE_AND_WRITE,
        0,
        &data->bytes[0],
        sizeof(data->bytes));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("could not write arbitrary %x", res);
        return res;
    }
    return 0;
}

static int _factory_write_config(void)
{
    int res_shielded = _setup_shielded_communication();
    if (res_shielded) {
        return res_shielded;
    }

    //
    // Configure AES key object
    //
    optiga_lib_status_t res = _optiga_util_write_metadata_sync(
        _util, OPTIGA_DATA_OBJECT_ID_AES_SYMKEY, aes_symkey_metadata, sizeof(aes_symkey_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("e200 metadata update failed: %x", res);
        return res;
    }

    //
    // Configure HMAC data object
    //
    rust_log("HMAC metadata");
    res = _optiga_util_write_metadata_sync(
        _util, OPTIGA_DATA_OBJECT_ID_HMAC, hmac_metadata, sizeof(hmac_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("HMAC metadata failed: %x", res);
        return res;
    }

    //
    // Configure arbitrary data object
    //
    rust_log("Arbitrary data metadata");
    res = _optiga_util_write_metadata_sync(
        _util,
        OPTIGA_DATA_OBJECT_ID_ARBITRARY_DATA,
        arbitrary_data_metadata,
        sizeof(arbitrary_data_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("Arbitrary data metadata failed: %x", res);
        return res;
    }
    // Initialize arbitrary data, all zeroes.
    const arbitrary_data_t arbitrary_data = {0};
    int write_res = _write_arbitrary_data(&arbitrary_data);
    if (write_res) {
        util_log("could not initialize arbitrary data");
        return write_res;
    }

    // Configure the monotonic counter.
    // Table 73, "Counter".
    // Bytes 0-3 are the initial counter value, set to 0.
    // Bytes 4-7 are the threshold.
    // Ints are encoded as uint32 big endian.
    uint8_t counter_buf[8] = {0};
    optiga_common_set_uint32(&counter_buf[4], MONOTONIC_COUNTER_MAX_USE);
    res = _optiga_util_write_data_sync(
        _util,
        OPTIGA_DATA_OBJECT_ID_COUNTER0,
        OPTIGA_UTIL_ERASE_AND_WRITE,
        0,
        counter_buf,
        sizeof(counter_buf));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: write initial counter data %x", res);
        return res;
    }

    //
    // Configure attestation key object
    //
    res = _optiga_util_write_metadata_sync(
        _util,
        OPTIGA_DATA_OBJECT_ID_ATTESTATION,
        attestation_metadata,
        sizeof(attestation_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("ECC metadata update failed: %x", res);
        return res;
    }

    util_log("write config OK");

    return 0;
}

static int _factory_setup(void)
{
    optiga_lib_status_t res;

    _util = optiga_util_create(OPTIGA_INSTANCE_ID_0, _optiga_lib_callback, NULL);
    if (NULL == _util) {
        util_log("couldn't create optiga util");
        return OPTIGA_ERR_CREATE;
    }

    _crypt = optiga_crypt_create(OPTIGA_INSTANCE_ID_0, _optiga_lib_callback, NULL);
    if (NULL == _crypt) {
        util_log("couldn't create optiga crypt");
        return OPTIGA_ERR_CREATE;
    }

    OPTIGA_UTIL_SET_COMMS_PROTOCOL_VERSION(_util, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);
    OPTIGA_CRYPT_SET_COMMS_PROTOCOL_VERSION(
        _crypt, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);

    OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(_util, OPTIGA_COMMS_NO_PROTECTION);
    res = _optiga_util_open_application_sync(_util, 0);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("failed to open util application: %x", res);
        return res;
    }

    res = _factory_write_config();
    if (res) {
        return res;
    }

    res = _optiga_util_close_application_sync(_util, 0);
    if (res != OPTIGA_LIB_SUCCESS) {
        return OPTIGA_ERR_CLOSE;
    }

    if (NULL != _crypt) {
        optiga_crypt_destroy(_crypt);
        _crypt = NULL;
    }

    if (NULL != _util) {
        optiga_util_destroy(_util);
        _util = NULL;
    }

    return 0;
}

static bool _verify_config(void)
{
    optiga_lib_status_t res;
    _util = optiga_util_create(OPTIGA_INSTANCE_ID_0, _optiga_lib_callback, NULL);
    if (NULL == _util) {
        return false;
    }

    _crypt = optiga_crypt_create(OPTIGA_INSTANCE_ID_0, _optiga_lib_callback, NULL);
    if (NULL == _crypt) {
        return false;
    }

    OPTIGA_UTIL_SET_COMMS_PROTOCOL_VERSION(_util, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);
    OPTIGA_CRYPT_SET_COMMS_PROTOCOL_VERSION(
        _crypt, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);

    // Verify shielded connection is active.
    if (_crypt->protection_level != OPTIGA_COMMS_FULL_PROTECTION) {
        util_log("crypt protection level expected to be FULL");
        return false;
    }
    if (_util->protection_level != OPTIGA_COMMS_FULL_PROTECTION) {
        util_log("util protection level expected to be FULL");
        return false;
    }

    res = _optiga_util_open_application_sync(_util, 0);
    if (res != OPTIGA_LIB_SUCCESS) {
        return false;
    }

    // TODO: verify metadata of each configured object.

    return true;
}

int optiga_setup(const securechip_interface_functions_t* ifs)
{
    if (ifs == NULL) {
        return OPTIGA_ERR_IFS;
    }
    _ifs = ifs;

    util_log("optiga_setup");

    // A timer is used to provide the OPTIGA library with the ability to schedule work on the main
    // event loop
    pal_timer_init();

#if true // FACTORYSETUP == 1
    int res = _factory_setup();
    if (res) {
        util_log("factory setup failed");
        return res;
    }
#endif

    if (!_verify_config()) {
        util_log("verify config failed");
        return OPTIGA_ERR_CONFIG_MISMATCH;
    }
    return 0;
}

bool optiga_update_keys(void)
{
    ABORT_IF_NULL(_ifs);
    ABORT_IF_NULL(_ifs->random_32_bytes);

    uint8_t new_key[32] = {0};
    _ifs->random_32_bytes(new_key);

    optiga_lib_status_t res = _optiga_util_write_data_sync(
        _util,
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
        _crypt, OPTIGA_SYMMETRIC_AES_256, OPTIGA_KEY_USAGE_ENCRYPTION, false, &keyid);
    util_log("genkey: %x", res);

    return res == OPTIGA_UTIL_SUCCESS;
}

int optiga_kdf_external(const uint8_t* msg, size_t len, uint8_t* mac_out)
{
    if (len != 32) {
        return OPTIGA_ERR_INVALID_ARGS;
    }

    ABORT_IF_NULL(_crypt);
    optiga_lib_status_t res;
    // The equivalient of python `mac_out = hmac.new(key, msg[:len], hashlib.sha256).digest()`

    uint32_t mac_out_len = 32;

    res = _optiga_crypt_hmac_sync(
        _crypt, OPTIGA_HMAC_SHA_256, OPTIGA_DATA_OBJECT_ID_HMAC, msg, len, mac_out, &mac_out_len);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("kdf fail err=%x", res);
        return res;
    }
    if (mac_out_len != 32) {
        return OPTIGA_ERR_UNEXPECTED_LEN;
    }

    return 0;
}

int optiga_kdf_internal(const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    if (len != 32) {
        return OPTIGA_ERR_INVALID_ARGS;
    }
    ABORT_IF_NULL(_crypt);
    optiga_lib_status_t res;

    uint8_t mac_out[16] = {0};
    uint32_t mac_out_len = sizeof(mac_out);

    res = _optiga_crypt_symmetric_encrypt_sync(
        _crypt,
        OPTIGA_SYMMETRIC_CMAC,
        OPTIGA_DATA_OBJECT_ID_AES_SYMKEY,
        msg,
        len,
        NULL,
        0,
        NULL,
        0,
        mac_out,
        &mac_out_len);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (mac_out_len != sizeof(mac_out)) {
        return OPTIGA_ERR_UNEXPECTED_LEN;
    }
    rust_sha256(mac_out, mac_out_len, kdf_out);
    return 0;
}

bool optiga_gen_attestation_key(uint8_t* pubkey_out)
{
    ABORT_IF_NULL(_crypt);
    optiga_key_id_t slot = OPTIGA_KEY_ID_E0F1;
    uint8_t pubkey_der[68] = {0};
    uint16_t pubkey_der_size = sizeof(pubkey_der);
    optiga_lib_status_t res = _optiga_crypt_ecc_generate_keypair_sync(
        _crypt,
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
    ABORT_IF_NULL(_crypt);
    uint8_t sig_der[70] = {0};
    uint16_t sig_der_size = sizeof(sig_der);
    optiga_lib_status_t res = _optiga_crypt_ecdsa_sign_sync(
        _crypt, challenge, 32, OPTIGA_KEY_ID_E0F1, sig_der, &sig_der_size);
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("sign failed: %x", res);
        return false;
    }
    // Parse signature, see Solution Reference Manual 6.2.2,
    // example for ECC NIST-P256 signature.
    // The R/S components are
    return rust_der_parse_optiga_signature(
        rust_util_bytes(sig_der, sig_der_size), rust_util_bytes_mut(signature_out, 64));
}

bool optiga_monotonic_increments_remaining(uint32_t* remaining_out)
{
    uint8_t buf[4] = {0};
    uint16_t size = sizeof(buf);
    optiga_lib_status_t res =
        _optiga_util_read_data_sync(_util, OPTIGA_DATA_OBJECT_ID_COUNTER0, 0, buf, &size);
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

// rand_out must be 32 bytes
bool optiga_random(uint8_t* rand_out)
{
    optiga_lib_status_t res = _optiga_crypt_random_sync(_crypt, OPTIGA_RNG_TYPE_TRNG, rand_out, 32);
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("optiga_random failed: %x", res);
        return false;
    }
    return true;
}

#if APP_U2F == 1 || FACTORYSETUP == 1
bool optiga_u2f_counter_set(uint32_t counter)
{
    arbitrary_data_t data = {0};
    if (!_read_arbitrary_data(&data)) {
        return false;
    }
    data.fields.u2f_counter = counter;
    return _write_arbitrary_data(&data) == 0;
}
#endif

#if APP_U2F == 1
bool optiga_u2f_counter_inc(uint32_t* counter)
{
    arbitrary_data_t data = {0};
    if (!_read_arbitrary_data(&data)) {
        return false;
    }
    data.fields.u2f_counter += 1;
    *counter = data.fields.u2f_counter;
    return _write_arbitrary_data(&data);
}
#endif

bool optiga_model(securechip_model_t* model_out)
{
    *model_out = OPTIGA_TRUST_M_V3;
    return true;
}
