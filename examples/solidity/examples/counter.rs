use revm::primitives::{Address, Bytes, U256};
use rustry_macros::{rustry_test, solidity};
use rustry_test::{
    common::contract::Contract,
    prelude::*,
    utils::{
        abi::{abi_decode, abi_encode_signature, AbiType},
        constants::bytes_zero,
    },
};

#[allow(unused)]
fn set_up() {
    let mut provider = Provider::default();

    let counter = solidity! {
        "
            // SPDX-License-Identifier: AGPLv3
            pragma solidity 0.8.20;

            contract Counter {
                uint256 public number;

                function setNumber(uint256 _number) public {
                    number = _number;
                }

                function increment() public {
                    number++;
                }
            }
        "
    };

    let mut counter = counter.deploy(&mut provider);
}

#[rustry_test(set_up)]
fn test_deployment() {
    assert_ne!(counter.address, Address::ZERO);
}

#[rustry_test(set_up)]
fn test_number() {
    let number = get_number(counter.address, &mut provider);
    assert_eq!(number, U256::ZERO);
}

#[rustry_test(set_up)]
fn test_increment() {
    provider
        .call(
            counter.address,
            abi_encode_signature("increment()", vec![]).into(),
        )
        .success();
    let number = get_number(counter.address, &mut provider);
    assert_eq!(number, U256::from(1));
}

#[rustry_test(set_up)]
fn test_increment_method() {
    counter.methods.increment(&mut provider);
    let number = counter.methods.number(&mut provider);
    assert_eq!(decode_number(&number), U256::from(1));
}

fn get_number(caddr: Address, provider: &mut Provider) -> U256 {
    let ret = provider.staticcall(caddr, abi_encode_signature("number()", vec![]).into());
    assert!(ret.is_success());
    let number = ret.get_data();
    decode_number(number)
}

fn decode_number(data: &Bytes) -> U256 {
    U256::from_be_bytes::<32>(abi_decode(data, vec![AbiType::Uint]).try_into().unwrap())
}

fn main() {}
