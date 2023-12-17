use revm::primitives::{alloy_primitives::Uint, Address, U256};
use rustry_macros::{huff, rustry_test};
use rustry_test::{
    common::contract::Contract,
    prelude::*,
    utils::{
        abi::{abi_decode, abi_encode_signature, AbiType},
        constants::bytes_zero,
    },
};
use hex;
fn set_up() {
    let mut provider = Provider::default();

    let simple_store = huff! {
        "
        /* Interface */
        #define function setValue(uint256) nonpayable returns ()
        #define function getValue() view returns (uint256)
        
        /* Storage Slots */
        #define constant VALUE_LOCATION = FREE_STORAGE_POINTER()
        
        /* Methods */
        #define macro SET_VALUE() = takes (1) returns (0) {
            0x04 calldataload   // [value]
            [VALUE_LOCATION]    // [ptr, value]
            sstore              // []
        }
        
        #define macro GET_VALUE() = takes (0) returns (0) {
            // Load value from storage.
            [VALUE_LOCATION]   // [ptr]
            sload              // [value]
            
        
            // Store value in memory.
            0x00 mstore
        
            // Return value
            0x20 0x00 return
        }
        
        #define macro MAIN() = takes (0) returns (0) {
            // Identify which function is being called.
            0x00 calldataload 0xE0 shr
            dup1 __FUNC_SIG(setValue) eq set jumpi
            dup1 __FUNC_SIG(getValue) eq get jumpi
        
            0x00 0x00 revert
        
            set:
                SET_VALUE()
            get:
                GET_VALUE()
        
        }
        "
    };

    let mut simple_store = simple_store.deploy(&mut provider);
}

#[rustry_test(set_up)]
fn test_deployment() {
    assert_ne!(simple_store.address, Address::ZERO);
}
#[rustry_test(set_up)]
fn test_set_value() {
    let number = 2;
    let value = Uint::<256, 4>::from(*&number);
    
    let data = abi_encode_signature("setValue(uint256)", vec![value.to_be_bytes::<32>().to_vec()]);

    provider
        .call(
            simple_store.address,
            data.into())
        .success();

    let number = get_value(simple_store.address, &mut provider);

    assert_eq!(number, U256::from(number));
}

fn get_value(caddr: Address, provider: &mut Provider) -> U256 {
    let ret = provider.staticcall(caddr, abi_encode_signature("getValue()", vec![]).into());
    assert!(ret.is_success());
    let data = ret.get_data();
    U256::from_be_bytes::<32>(abi_decode(data, vec![AbiType::Uint]).try_into().unwrap())
}

fn main() {}
