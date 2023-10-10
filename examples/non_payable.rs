use rustry::huff;

fn set_up() {
    let mut provider = Provider::default();

    let non_payable = huff! {
        r#"
            #define macro MAIN() = takes(0) returns(0) {
                callvalue iszero no_value jumpi
                0x00 0x00 revert
                no_value:
                    stop
            }
        "#
    }
}
