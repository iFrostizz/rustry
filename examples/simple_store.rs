use revm::primitives::Address;
use rustry_macros::{rustry_test, solidity};
use rustry_test::{provider::db::Frontend, Provider};

#[allow(unused)]
fn set_up() {
    let mut provider = Provider::default();

    let counter = solidity! {
        "
            // SPDX-License-Identifier: AGPLv3
            pragma solidity 0.8.20;

            contract SimpleStore {
                uint256 public number;

                function setNumber(uint256 _number) public {
                    number = _number;
                }
            }
        "
    };

    let counter = counter.deploy(&mut provider);
}

#[rustry_test(set_up)]
fn test_init_number() {
    assert_eq!(counter.number(), 0);
}

#[rustry_test(set_up)]
fn test_number() {
    counter.methods.setNumber(&mut provider, 69);
    assert_eq!(counter.methods.number(&mut provider), 69);
}
