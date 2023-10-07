use rustry::*;

fn set_up() {
    let val = 2;
    let counter = solidity! {
        "
        pragma solidity ^0.8.22;

        contract Counter {
            uint256 public number;

            function setNumber(uint256 _number) external {
                number = _number;
            }

            function increment() external {
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
