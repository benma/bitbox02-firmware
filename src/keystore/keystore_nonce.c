#include "keystore_nonce.h"
#include "keystore.h"

#include <hardfault.h>
#include <util.h>

typedef struct {
    uint32_t keypath[10];
    size_t keypath_len;
    uint8_t msg[32];
} _signdata_t;

static _signdata_t _signdata;
static bool _has_signdata;

bool keystore_nonce_secp256k1_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* client_commitment_out)
{
    if (keypath_len > sizeof(_signdata.keypath) / sizeof(uint32_t)) {
        Abort("keystore_nonce_secp256k1_commit: keypath too long");
    }
    if (_has_signdata) {
        return false;
    }

    if (!keystore_secp256k1_nonce_commit(
            keypath, keypath_len, msg32, host_commitment, client_commitment_out)) {
        return false;
    }
    memcpy(_signdata.keypath, keypath, keypath_len * sizeof(uint32_t));
    _signdata.keypath_len = keypath_len;
    memcpy(_signdata.msg, msg32, sizeof(_signdata.msg));
    _has_signdata = true;
    return true;
}

bool keystore_nonce_secp256k1_sign(
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out)
{
    if (!_has_signdata) {
        return false;
    }
    bool result = keystore_secp256k1_sign(
        _signdata.keypath,
        _signdata.keypath_len,
        _signdata.msg,
        host_nonce32,
        sig_compact_out,
        recid_out);
    keystore_nonce_clear();
    return result;
}

void keystore_nonce_clear(void)
{
    _has_signdata = false;
    util_zero(&_signdata, sizeof(_signdata));
}
