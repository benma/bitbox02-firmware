// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;

use serde::Deserialize;

use bitbox02_rust::hww::api::ethereum::sighash::{
    ChunkingProducer, ParamsEIP1559, ParamsLegacy, SimpleProducer, compute_eip1559, compute_legacy,
    tests::clear_chunk_responder, tests::setup_chunk_responder,
};
use util::bb02_async::block_on;

fn decode_hex(s: &str) -> Vec<u8> {
    hex::decode(s).unwrap()
}

#[derive(Debug, Deserialize)]
struct Eip1559TestCase {
    chain_id: u64,
    nonce: String,
    max_priority_fee: String,
    max_fee_per_gas: String,
    gas_limit: String,
    recipient: String,
    value: String,
    data: String,
    expected_sighash: String,
}

#[derive(Debug, Deserialize)]
struct LegacyTestCase {
    chain_id: u64,
    nonce: String,
    gas_price: String,
    gas_limit: String,
    recipient: String,
    value: String,
    data: String,
    expected_sighash: String,
}

#[test]
fn test_compute_eip1559() {
    let json_data = include_str!("../src/hww/api/ethereum/testdata/eip1559_tests.json");
    let tests: Vec<Eip1559TestCase> = serde_json::from_str(json_data).unwrap();

    const DATA_THRESHOLD: usize = 6144;

    for (i, test) in tests.iter().enumerate() {
        let nonce = decode_hex(&test.nonce);
        let max_priority_fee = decode_hex(&test.max_priority_fee);
        let max_fee_per_gas = decode_hex(&test.max_fee_per_gas);
        let gas_limit = decode_hex(&test.gas_limit);
        let recipient = decode_hex(&test.recipient);
        let value = decode_hex(&test.value);
        let data = decode_hex(&test.data);
        let expected_sighash: [u8; 32] = decode_hex(&test.expected_sighash).try_into().unwrap();

        if data.len() < DATA_THRESHOLD {
            let params = ParamsEIP1559 {
                chain_id: test.chain_id,
                nonce: &nonce,
                max_priority_fee_per_gas: &max_priority_fee,
                max_fee_per_gas: &max_fee_per_gas,
                gas_limit: &gas_limit,
                recipient: &recipient,
                value: &value,
                data: RefCell::new(SimpleProducer::new(&data)),
            };
            let result = block_on(compute_eip1559(&params)).unwrap();
            assert_eq!(
                result, expected_sighash,
                "EIP1559 test {} failed (SimpleProducer)",
                i
            );
        } else {
            setup_chunk_responder(data.clone());
            let params = ParamsEIP1559 {
                chain_id: test.chain_id,
                nonce: &nonce,
                max_priority_fee_per_gas: &max_priority_fee,
                max_fee_per_gas: &max_fee_per_gas,
                gas_limit: &gas_limit,
                recipient: &recipient,
                value: &value,
                data: RefCell::new(ChunkingProducer::new(data.len())),
            };
            let result = block_on(compute_eip1559(&params)).unwrap();
            assert_eq!(
                result, expected_sighash,
                "EIP1559 test {} failed (ChunkingProducer)",
                i
            );
            clear_chunk_responder();
        }
    }
}

#[test]
fn test_compute_legacy() {
    let json_data = include_str!("../src/hww/api/ethereum/testdata/legacy_tests.json");
    let tests: Vec<LegacyTestCase> = serde_json::from_str(json_data).unwrap();

    const DATA_THRESHOLD: usize = 6144;

    for (i, test) in tests.iter().enumerate() {
        let nonce = decode_hex(&test.nonce);
        let gas_price = decode_hex(&test.gas_price);
        let gas_limit = decode_hex(&test.gas_limit);
        let recipient = decode_hex(&test.recipient);
        let value = decode_hex(&test.value);
        let data = decode_hex(&test.data);
        let expected_sighash: [u8; 32] = decode_hex(&test.expected_sighash).try_into().unwrap();

        if data.len() < DATA_THRESHOLD {
            let params = ParamsLegacy {
                nonce: &nonce,
                gas_price: &gas_price,
                gas_limit: &gas_limit,
                recipient: &recipient,
                value: &value,
                data: RefCell::new(SimpleProducer::new(&data)),
                chain_id: test.chain_id,
            };
            let result = block_on(compute_legacy(&params)).unwrap();
            assert_eq!(
                result, expected_sighash,
                "Legacy test {} failed (SimpleProducer)",
                i
            );
        } else {
            setup_chunk_responder(data.clone());
            let params = ParamsLegacy {
                nonce: &nonce,
                gas_price: &gas_price,
                gas_limit: &gas_limit,
                recipient: &recipient,
                value: &value,
                data: RefCell::new(ChunkingProducer::new(data.len())),
                chain_id: test.chain_id,
            };
            let result = block_on(compute_legacy(&params)).unwrap();
            assert_eq!(
                result, expected_sighash,
                "Legacy test {} failed (ChunkingProducer)",
                i
            );
            clear_chunk_responder();
        }
    }
}
