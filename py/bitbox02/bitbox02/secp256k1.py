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


def pubkey_serialize_compressed(pubkey: ecdsa.VerifyingKey) -> bytes:
    """
    Serializes a secp256k1 public key to 33 bytes compressed format (y sign and x-coordinate).
    """
    assert pubkey.curve is ecdsa.curves.SECP256k1
    is_odd = pubkey.pubkey.point.y() % 2 == 1
    x_bytes: bytes = pubkey.pubkey.point.x().to_bytes(32, "big")
    return (b"\x03" if is_odd else b"\x02") + x_bytes


class ECDSANonceException(Exception):
    pass


def anti_nonce_covert_channel_verify(
    host_nonce: bytes, client_nonce_commitment: bytes, signature: bytes
) -> None:
    """
    Verifies that hostNonce was used to tweak the nonce during signature
    generation according to k' = k + H(clientCommitment, hostNonce) by checking that
    k'*G = clientCommitment + H(clientCommitment, hostNonce)*G.
    Throws ECDSANonceException if the verification fails.
    """
    assert len(host_nonce) == 32
    assert (
        len(client_nonce_commitment) == 65 and client_nonce_commitment[0] == 0x4
    ), "expected uncompressed pubkey"
    assert len(signature) == 64
    client_nonce_commitment_pubkey = ecdsa.VerifyingKey.from_string(
        client_nonce_commitment[1:], ecdsa.curves.SECP256k1
    )
    # Compute R = R1 + H(R1, host_nonce)*G. R1 is the client nonce commitment, which we get
    # uncompressed, but need to hash using the compressed serialization.
    tweak = hashlib.sha256(
        pubkey_serialize_compressed(client_nonce_commitment_pubkey) + host_nonce
    ).digest()
    tweak_pubkey = ecdsa.SigningKey.from_string(tweak, curve=ecdsa.curves.SECP256k1).verifying_key
    tweaked_nonce = tweak_pubkey.pubkey.point + client_nonce_commitment_pubkey.pubkey.point
    expected_sig_r = tweaked_nonce.x() % ecdsa.curves.SECP256k1.order
    sig_r = int.from_bytes(signature[:32], "big")
    if sig_r != expected_sig_r:
        raise ECDSANonceException(
            "Could not verify that the host nonce was contributed to the client nonce"
        )
