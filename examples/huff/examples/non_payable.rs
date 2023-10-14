use revm::primitives::{alloy_primitives::Uint, Address};
use rustry_macros::{huff, rustry_test};
use rustry_test::{common::contract::Contract, prelude::*, utils::constants::ether};

fn set_up() {
    let mut provider = Provider::default();

    let non_payable = huff! {
        "
            #define macro MAIN() = takes(0) returns(0) {
                callvalue iszero no_value jumpi
                0x00 0x00 revert
                no_value:
                    stop
            }
        "
    };

    let mut non_payable = non_payable.deploy(&mut provider);
}

#[rustry_test(set_up)]
fn test_deployment() {
    assert_ne!(non_payable.address, Address::ZERO);
}

#[rustry_test(set_up)]
fn test_novalue() {
    assert!(provider
        .call(non_payable.address, vec![0].into())
        .is_success());
}

#[rustry_test(set_up)]
fn test_value() {
    provider.mint(ether(), Address::ZERO);
    assert!(!provider
        .send(non_payable.address, Uint::from(10).into())
        .is_success());
}

fn main() {}
