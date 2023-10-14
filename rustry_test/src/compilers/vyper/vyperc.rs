use crate::compilers::{
    builder::{CompilerError, CompilerOutput, RunCompiler},
    solidity::solc::{JsonError, OutError, Source, StorageLayout},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::Write,
    process::{Command, Stdio},
};

#[derive(Clone, Serialize)]
pub struct Settings {
    #[serde(rename = "evmVersion")]
    pub evm_version: String,
    #[serde(rename = "outputSelection")]
    pub output_selection: HashMap<String, HashMap<String, Vec<OutputOption>>>,
}

// TODO move as well
#[derive(Clone, Serialize)]
pub enum OutputOption {
    #[serde(rename = "evm.bytecode")]
    EvmBytecode,
    #[serde(rename = "abi")]
    Abi,
}

// https://docs.vyperlang.org/en/stable/compiling-a-contract.html?highlight=--standard-json#input-json-description
#[derive(Builder, Serialize)]
pub struct Vyperc {
    #[builder(setter(skip))]
    #[builder(default = "String::from(\"Vyper\")")]
    pub language: String,
    #[builder(setter(skip))]
    pub sources: HashMap<String, Source>,
    #[builder(default = "None")]
    pub settings: Option<Settings>,
    // pub interfaces: ...
}

#[derive(Debug, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub enum ErrType {
    AttributeError,
    JSONError,
}

fn constructor() -> String {
    String::from("constructor")
}

#[derive(Debug, Deserialize)]
pub struct BytecodeData {
    pub object: String,
    pub opcodes: String,
}

#[derive(Debug, Deserialize)]
pub struct DeployedBytecodeData {
    #[serde(flatten)]
    pub bytecode_data: BytecodeData,
    #[serde(rename = "immutableReferences")]
    pub immutable_references: HashMap<String, String>, // TODO
}

#[derive(Debug, Deserialize)]
pub struct EvmOutput {
    pub assembly: Option<String>,
    #[serde(rename = "legacyAssembly")]
    pub legacy_assembly: Option<HashMap<String, String>>,
    pub bytecode: Option<BytecodeData>,
    #[serde(rename = "deployedBytecode")]
    pub deployed_bytecode: Option<DeployedBytecodeData>,
    #[serde(rename = "methodIdentifiers")]
    pub method_identifiers: Option<HashMap<String, String>>,
    #[serde(rename = "gasEstimates")]
    pub gas_estimates: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug, Deserialize)]
pub struct AbiEntry {
    #[serde(default = "constructor")]
    pub name: String,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub inputs: Option<Vec<AbiEntry>>,
    pub outputs: Option<Vec<AbiEntry>>,
    #[serde(rename = "stateMutability")]
    pub state_mutability: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VycContract {
    pub abi: Option<Vec<AbiEntry>>,
    #[serde(rename = "userdoc")]
    pub user_doc: Option<HashMap<String, String>>,
    #[serde(rename = "devdoc")]
    pub dev_doc: Option<HashMap<String, String>>,
    pub ir: Option<String>,
    #[serde(rename = "irAst")]
    pub ir_ast: Option<HashMap<String, String>>,
    #[serde(rename = "irOptimized")]
    pub ir_optimized: Option<String>,
    #[serde(rename = "irOptimizedAst")]
    pub ir_optimized_ast: Option<HashMap<String, String>>,
    #[serde(rename = "storageLayout")]
    pub storage_layout: Option<StorageLayout>,
    pub evm: Option<EvmOutput>,
}

// https://docs.vyperlang.org/en/stable/compiling-a-contract.html?highlight=--standard-json#output-json-description
#[derive(Debug, Deserialize)]
pub struct VypercOut {
    pub compiler: String,
    pub errors: Option<Vec<OutError<SourceLocation, ErrType>>>,
    pub sources: Option<HashMap<String, HashMap<String, i32>>>,
    pub contracts: Option<HashMap<String, HashMap<String, VycContract>>>,
}

impl From<VypercOut> for CompilerOutput {
    fn from(val: VypercOut) -> Self {
        CompilerOutput::Vyper(val)
    }
}

impl TryFrom<CompilerOutput> for VypercOut {
    type Error = String; // TODO not very idiomatic

    fn try_from(value: CompilerOutput) -> Result<VypercOut, String> {
        match value {
            CompilerOutput::Vyper(vyc_out) => Ok(vyc_out),
            _ => Err(String::from("Error with TryFrom !")),
        }
    }
}

impl RunCompiler for Vyperc {
    fn run(&self) -> Result<CompilerOutput, CompilerError> {
        let input = serde_json::to_string(&self).unwrap();

        let mut child = Command::new("vyper")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .arg("--standard-json")
            .spawn()
            .unwrap();

        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(input.as_bytes()).unwrap();
        let output = child.wait_with_output().unwrap();

        let stdout = output.stdout;
        let raw_out = String::from_utf8(stdout).unwrap();

        if !output.status.success() {
            return Err(JsonError {
                message: String::from_utf8(output.stderr).unwrap(),
            }
            .into());
        }

        let vyc_out = if let Ok(vyc_out) = serde_json::from_str::<VypercOut>(&raw_out) {
            vyc_out
        } else {
            panic!("failed to deserialize vyc output: {}", &raw_out);
        };

        if let Some(errs) = &vyc_out.errors {
            if !errs.is_empty() {
                let first_err = errs.first().unwrap();
                let message = first_err
                    .formatted_message
                    .as_ref()
                    .unwrap_or(&first_err.message);
                Err(JsonError {
                    message: message.to_string(),
                }
                .into())
            } else {
                Ok(vyc_out.into())
            }
        } else {
            Ok(vyc_out.into())
        }
    }
}
