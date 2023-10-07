use rustry::*;

fn set_up() {
    let val = 2;
    let counter = solidity! {
        "
            // SPDX-License-Identifier: MIT
            pragma solidity 0.8.20;
        "
    };
}

#[rustry_test(set_up)]
fn test_thing() {
    assert_eq!(val + 2, 4);
}
