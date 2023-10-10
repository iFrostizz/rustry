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

    let counter = counter.deploy(&mut provider);
}

#[rustry_test(set_up)]
fn test_deployment() {
    assert_ne!(counter.address, Address::ZERO);
}
