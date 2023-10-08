use rustry_macros::rustry_test;
use rustry_test::Provider;

fn set_up() {
    let val = 2;
    let provider = Provider::default();
    let counter = solidity! {
        "
            // SPDX-License-Identifier: MIT
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
}

#[rustry_test(set_up)]
fn test_thing() {
    assert_eq!(val + 2, 4);
}
