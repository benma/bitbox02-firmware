# Copyright 2019 Shift Cryptosecurity AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""secp256k1 util functions"""
import hashlib

import ecdsa


class ECDSANonceException(Exception):
    pass


def antiklepto_host_commit(host_nonce: bytes) -> bytes:
    tag = hashlib.sha256(b"s2c/ecdsa/data").digest()
    return hashlib.sha256(tag + tag + host_nonce).digest()


def antiklepto_verify(host_nonce: bytes, signer_commitment: bytes, signature: bytes) -> None:
    """
    Verifies that hostNonce was used to tweak the nonce during signature
    generation according to k' = k + H(signerCommitment, hostNonce) by checking that
    k'*G = signerCommitment + H(signerCommitment, hostNonce)*G.
    Throws ECDSANonceException if the verification fails.
    """
    assert len(host_nonce) == 32
    assert len(signer_commitment) == 33, "expected uncompressed pubkey"
    assert len(signature) == 64
    signer_commitment_pubkey = ecdsa.VerifyingKey.from_string(
        signer_commitment, ecdsa.curves.SECP256k1
    )
    # Compute R = R1 + H(R1, host_nonce)*G. R1 is the client nonce commitment, which we get
    # uncompressed, but need to hash using the compressed serialization.
    tag = hashlib.sha256(b"s2c/ecdsa/point").digest()
    tweak = hashlib.sha256(tag + tag + signer_commitment + host_nonce).digest()
    tweak_pubkey = ecdsa.SigningKey.from_string(tweak, curve=ecdsa.curves.SECP256k1).verifying_key
    tweaked_nonce = tweak_pubkey.pubkey.point + signer_commitment_pubkey.pubkey.point
    expected_sig_r = tweaked_nonce.x() % ecdsa.curves.SECP256k1.order
    sig_r = int.from_bytes(signature[:32], "big")
    if sig_r != expected_sig_r:
        raise ECDSANonceException(
            "Could not verify that the host nonce was contributed to the client nonce"
        )
