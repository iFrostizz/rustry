#![feature(proc_macro_span)]

mod common;
mod compilers;
mod harness;

use crate::compilers::{
    builder::{BinError, Compiler, CompilerError, CompilerKinds},
    solidity::{solc::SolcOut, types::internal_to_type}, // reexport solidity or rename
};
use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::{collections::HashMap, iter};
use syn::{parse_macro_input, Error, ItemFn};

use self::compilers::{huff::huffc::HuffcOut, solidity::solc::AbiEntry};

/// # Examples
///
/// ```
/// use rustry_macros::rustry_test;
///
/// fn set_up() {
///     // let counter = deploy_contract("src/Counter.sol:Counter");
/// }
///
/// #[rustry_test(set_up)]
/// fn test_increment() {
///     // if annotated with `#[rustry_test]` and that there is a set_up function,
///     // the content of the `set_up` will be copy/pasted to each rustry_test.
///     // counter.increment().send().await;
///     // assert_eq!(counter.number(), 1);
/// }
///
/// #[rustry_test(set_up)]
/// fn testFuzz_set_number(x: U256) {
///     // counter.setNumber(x).send().await;
///     // assert_eq!(counter.number(), x);
/// }
/// ```
#[proc_macro_attribute]
pub fn rustry_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut set_up_name = None;
    let set_up_parser = syn::meta::parser(|meta| {
        if set_up_name.is_some() {
            eprintln!("{:?}", set_up_name.clone().into_token_stream().to_string());
            return Err(Error::new_spanned(
                args.clone().to_string(),
                "should have only one function name",
            ));
        } else {
            set_up_name = Some(meta.path);
        }
        Ok(())
    });
    let ar = args.clone();
    parse_macro_input!(ar with set_up_parser);

    let span = Span::call_site();
    let macro_path = span.source_file().path().canonicalize().unwrap();
    let code = std::fs::read_to_string(macro_path).unwrap();
    let syntax = syn::parse_file(&code).unwrap();
    let set_up_block = if let Some(fname) = set_up_name {
        if let Some(set_up_fn) = syntax.items.into_iter().find(|item| {
            if let syn::Item::Fn(_fn) = item {
                _fn.sig.ident == fname.clone().into_token_stream().to_string()
            } else {
                false
            }
        }) {
            match set_up_fn {
                syn::Item::Fn(syn::ItemFn { block, .. }) => {
                    let block: syn::Block = *block;
                    let stmts = block.stmts;
                    quote! {
                        #(#stmts)*
                    }
                }
                _ => unreachable!(),
            }
        } else {
            proc_macro2::TokenStream::new()
        }
    } else {
        proc_macro2::TokenStream::new()
    };
    let fun = parse_macro_input!(input as ItemFn);
    let fname = fun.sig.ident;
    let block = fun.block;

    let def = default_set_up();

    quote! {
        // #[tokio::test]
        #[test]
        pub fn #fname() {
            #def
            #set_up_block
            #block
        }
    }
    .into()
}

// TODO figure out the source mappings
#[proc_macro]
pub fn solidity(input: TokenStream) -> TokenStream {
    let lit_str = parse_macro_input!(input as syn::LitStr);
    let source_code = lit_str.value();

    let solc = Compiler {
        kind: CompilerKinds::Solc,
        sources: HashMap::from([(String::from("source_code.sol"), source_code.clone())]),
    };

    match solc.run() {
        Ok(out) => {
            let solc_out = SolcOut::try_from(out).unwrap();
            let contracts = solc_out.contracts.unwrap();
            let contract = contracts
                .get("source_code.sol")
                .unwrap()
                .get("Counter")
                .unwrap();

            let bytecode = &contract
                .evm
                .as_ref()
                .unwrap()
                .bytecode
                .as_ref()
                .unwrap()
                .object;

            let functions: Vec<_> = contract
                .abi
                .as_ref()
                .unwrap()
                .iter()
                .filter(|entry| entry.entry_type == "function")
                .collect();

            let impl_fns = functions.iter().map(|func| {
                let name: proc_macro2::TokenStream = func.name.parse().unwrap();
                let inputs_w_types = func.inputs.iter().map(|input| {
                    let iname: proc_macro2::TokenStream = input.name.clone().parse().unwrap();
                    let itype: proc_macro2::TokenStream =
                        internal_to_type(&input.type_type).parse().unwrap();
                    quote! {
                        #iname: #itype
                    }
                });

                let outputs = func.outputs.iter().map(|output| {
                    let otype: proc_macro2::TokenStream =
                        internal_to_type(&output.type_type).parse().unwrap();
                    quote! {
                        #otype
                    }
                });

                let fn_call = match func.state_mutability.as_str() {
                    "nonpayable" => proc_macro2::TokenStream::new(),
                    "view" => proc_macro2::TokenStream::new(),
                    _ => unimplemented!(),
                };
                let fn_ret = func.outputs.iter().map(|_| 0u128);

                quote! {
                    #[allow(clippy::unused_unit)]
                    pub fn #name(
                        &self,
                        provider: &mut rustry_test::Provider,
                        #(#inputs_w_types),*
                    ) -> (#(#outputs),*) {
                        #fn_call

                        (#(#fn_ret),*)
                    }
                }
            });

            make_contract_instance(impl_fns, bytecode)
        }
        Err(err) => match err {
            CompilerError::BuilderError(_) => todo!(),
            CompilerError::BinError(err) => match err {
                BinError::Solc(solc_err) => {
                    Error::new_spanned(source_code, solc_err.message).to_compile_error()
                }
            },
        },
    }
    .into()
}

#[proc_macro]
pub fn huff(input: TokenStream) -> TokenStream {
    let lit_str = parse_macro_input!(input as syn::LitStr);
    let source_code = lit_str.value();

    let huffc = Compiler {
        kind: CompilerKinds::Huff,
        sources: HashMap::from([(String::from("source_code.sol"), source_code.clone())]),
    };

    match huffc.run() {
        Ok(out) => {
            let huffc_out = HuffcOut::try_from(out).unwrap();
            let bytecode = huffc_out.bytecode;

            make_contract_instance(iter::empty::<proc_macro2::TokenStream>(), &bytecode)
        }
        Err(err) => panic!("{:?}", err),
    }
    .into()
}

fn default_set_up() -> proc_macro2::TokenStream {
    quote! {
        let provider = 0;
    }
}

fn make_contract_instance(
    impl_fns: impl Iterator<Item = proc_macro2::TokenStream>,
    bytecode: &String,
) -> proc_macro2::TokenStream {
    quote! {
        {
            #[derive(Default, Debug)]
            struct ContractMethods;

            impl ContractMethods {
                #(
                    #impl_fns
                 )*
            }

            #[derive(Default, Debug)]
            struct ContractInstance {
                pub code: revm::primitives::Bytes,
            }

            #[derive(Default, Debug)]
            struct DeployedContract {
                pub address: Address,
                pub methods: ContractMethods
            }

            impl ContractInstance {
                fn new(code: revm::primitives::Bytes) -> Self {
                    Self {
                        code,
                    }
                }

                fn deploy(self, provider: &mut rustry_test::Provider) -> DeployedContract {
                    let address = provider.deploy(self.code).unwrap();
                    DeployedContract {
                        address,
                        methods: ContractMethods::default()
                    }
                }
            }

            let as_bytes = hex::decode(#bytecode).unwrap();

            let _bytecode: revm::primitives::Bytes = as_bytes.into();

            ContractInstance::new(_bytecode)
        }
    }
}
