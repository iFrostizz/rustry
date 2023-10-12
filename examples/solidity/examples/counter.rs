use revm::primitives::{Bytes, Address, U256};
use rustry_macros::{rustry_test, solidity};
use rustry_test::common::contract::Contract;
use rustry_test::utils::abi::abi_encode_signature;
use rustry_test::{provider::db::{ExecRes, Frontend}, utils::constants::bytes_zero, Provider};

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
    let number = get_number(counter);
    assert_eq!(number, U256::ZERO);
}

#[rustry_test(set_up)]
fn test_increment() {
    counter
        .call(abi_encode_signature("increment()", vec![]))
        .success();
    let number = get_number(counter);
    assert_eq!(number, U256::from(1));
}

fn get_number<T: Contract>(mut counter: T) -> U256 {
    let ret = counter.staticcall(abi_encode_signature("number()", vec![]));
    assert!(ret.is_success());
    let number = ret.get_data();
    let num_vec = number.to_vec();
    assert!(num_vec.len() == 32);
    let arr: [u8; 32] = number.to_vec().try_into().unwrap();
    U256::from_be_bytes(arr)
}

fn main() {}
