use revm::primitives::Address;
use rustry_macros::{huff, rustry_test};
use rustry_test::{provider::db::Frontend, Provider};

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

    let non_payable = non_payable.deploy(&mut provider);
}

#[rustry_test(set_up)]
fn test_deployment() {
    assert_ne!(non_payable.address, Address::ZERO);
}
