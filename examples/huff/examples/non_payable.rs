use revm::primitives::{alloy_primitives::Uint, Address};
use rustry_macros::{huff, rustry_test};
use rustry_test::{
    provider::db::{Cheats, Frontend},
    utils::constants::ether,
    Provider,
};

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
    assert!(non_payable.call(vec![0].into()).is_success());
}

#[rustry_test(set_up)]
fn test_value() {
    non_payable.provider.mint(ether(), Address::ZERO);
    assert!(!non_payable.send(Uint::from(10)).is_success());
}

fn main() {}
