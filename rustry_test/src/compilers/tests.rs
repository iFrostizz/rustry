use super::huff::huffc::HuffcOut;
use super::solidity::solc::SolcOut;
use super::vyper::vyperc::VypercOut;
use std::{fs, path::PathBuf};

#[test]
fn deserialize_solc_output() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/test-data/solc/output/");

    for dir_entry in fs::read_dir(d).unwrap() {
        let file = dir_entry.unwrap().path();
        let content = fs::read_to_string(file).unwrap();
        serde_json::from_str::<SolcOut>(&content).unwrap();
    }
}

#[test]
fn deserialize_huffc_output() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/test-data/huffc/output/");

    for dir_entry in fs::read_dir(d).unwrap() {
        let file = dir_entry.unwrap().path();
        let content = fs::read_to_string(file).unwrap();
        serde_json::from_str::<HuffcOut>(&content).unwrap();
    }
}

#[test]
fn deserialize_vyper_output() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/test-data/vyperc/output/");

    for dir_entry in fs::read_dir(d).unwrap() {
        let file = dir_entry.unwrap().path();
        let content = fs::read_to_string(file).unwrap();
        serde_json::from_str::<VypercOut>(&content).unwrap();
    }
}
