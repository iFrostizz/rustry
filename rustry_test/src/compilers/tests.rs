use super::{huff::huffc::HuffcOut, solidity::solc::SolcOut, vyper::vyperc::VypercOut};
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[test]
fn deserialize_solc_output() {
    test_compiler_output::<SolcOut>("solc");
}

#[test]
fn deserialize_huffc_output() {
    test_compiler_output::<HuffcOut>("huffc");
}

#[test]
fn deserialize_vyper_output() {
    test_compiler_output::<VypercOut>("vyperc");
}

fn test_compiler_output<T: for<'a> Deserialize<'a>>(compiler: &str) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("src/test-data/{compiler}/output/"));

    // println!("Compiler: {compiler}");
    for dir_entry in fs::read_dir(d).unwrap() {
        let dir_entry = dir_entry.unwrap();
        let file = dir_entry.path();
        let file_name = dir_entry.file_name().into_string().unwrap();
        let content = fs::read_to_string(file).unwrap().clone();
        let content: &str = &content;
        println!("{}", file_name);
        serde_json::from_str::<T>(content).unwrap();
    }
    println!();
}
