#![feature(proc_macro_span)]

mod compilers;
mod harness;
mod utils;

use std::collections::HashMap;

use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Error, ItemFn};

use crate::compilers::builder::{BinError, Compiler, CompilerError, CompilerKinds};

/// # Examples
///
/// ```
/// use rustry::rustry_test;
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
            dbg!(&out);
            quote! {
                0
            }
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

fn default_set_up() -> proc_macro2::TokenStream {
    quote! {
        let provider = 0;
    }
}
