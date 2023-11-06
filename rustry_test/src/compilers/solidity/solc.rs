use crate::{
    common::utils::opt_false,
    compilers::builder::{BinError, CompilerError, CompilerOutput, RunCompiler},
};
use core::fmt;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    process::{Command, Stdio},
};

// TODO we should move that in a common place
#[derive(Serialize)]
pub struct Source {
    pub content: String,
}

// TODO move as well
#[derive(Clone, Serialize)]
pub enum OutputOption {
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "evm.bytecode")]
    EvmBytecode,
    #[serde(rename = "abi")]
    Abi,
}

#[derive(Clone, Serialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "opt_none")]
    pub remappings: Option<Vec<String>>,
    #[serde(rename = "outputSelection")]
    pub output_selection: HashMap<String, HashMap<String, Vec<OutputOption>>>,
}

// https://docs.soliditylang.org/en/latest/using-the-compiler.html#input-description
#[derive(Builder, Serialize)]
pub struct Solc {
    #[builder(default = "String::from(\"Solidity\")")]
    pub language: String,
    #[builder(setter(skip))]
    pub sources: HashMap<String, Source>,
    // #[serde(rename(serialize = "--bin"))]
    #[serde(skip_serializing)]
    #[builder(default = "false")]
    pub bin: bool,
    // #[serde(rename(serialize = "--ast-compact-json"))]
    #[serde(skip_serializing_if = "opt_false")]
    #[builder(default = "false")]
    pub ast_compact_json: bool,
    // #[serde(rename(serialize = "--asm"))]
    #[serde(skip_serializing_if = "opt_false")]
    #[builder(default = "false")]
    pub asm: bool,
    #[builder(default = "None")]
    pub settings: Option<Settings>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceLocation {
    file: String,
    start: i32,
    end: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrType {
    TypeError,
    InternalCompilerError,
    Exception,
    JSONError,
    ParserError,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutError<S, E> {
    #[serde(rename = "sourceLocation")]
    pub source_location: Option<S>,
    #[serde(rename = "secondarySourceLocation")]
    pub secondary_source_locations: Option<Vec<SourceLocation>>,
    #[serde(rename = "type")]
    pub err_type: E,
    pub component: String,
    pub severity: Severity,
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,
    pub message: String,
    #[serde(rename = "formattedMessage")]
    pub formatted_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugData {
    #[serde(rename = "entryPoint")]
    pub entry_point: i32,
    pub id: Option<i32>,
    #[serde(rename = "parameterSlots")]
    pub parameter_slots: Option<i32>,
    #[serde(rename = "returnSlots")]
    pub return_slots: Option<i32>,
}

// TODO
type Ast = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedSource {
    pub ast: Ast,
    pub contents: String,
    pub id: i32,
    pub language: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BytecodeData {
    #[serde(rename = "functionDebugData")]
    pub function_debug_data: HashMap<String, DebugData>,
    pub object: String,
    pub opcodes: String,
    #[serde(rename = "sourceMap")]
    pub source_map: String,
    #[serde(rename = "generatedSources")]
    pub generated_sources: Vec<GeneratedSource>,
    #[serde(rename = "linkReferences")]
    pub link_references: HashMap<String, HashMap<String, Vec<SourceLocation>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedBytecodeData {
    #[serde(flatten)]
    pub bytecode_data: BytecodeData,
    #[serde(rename = "immutableReferences")]
    pub immutable_references: HashMap<String, String>, // TODO
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLayout {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeType {
    #[serde(rename = "internalType")]
    pub internal_type: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_type: String,
}

// TODO move elsewhere
pub trait EntryUtils {
    fn signature(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbiEntry {
    pub inputs: Vec<TypeType>,
    pub name: String,
    pub outputs: Vec<TypeType>,
    #[serde(rename = "stateMutability")]
    pub state_mutability: String,
    #[serde(rename = "type")]
    pub entry_type: String,
}

impl EntryUtils for AbiEntry {
    fn signature(&self) -> String {
        let mut inner_types: String = self
            .inputs
            .iter()
            .map(|tt| format!("{},", tt.type_type.clone()))
            .collect();
        inner_types.pop();
        format!("{0}({inner_types})", self.name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolcContract {
    pub abi: Option<Vec<AbiEntry>>,
    pub metadata: Option<String>,
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

// https://docs.soliditylang.org/en/latest/using-the-compiler.html#output-description
#[derive(Debug, Serialize, Deserialize)]
pub struct SolcOut {
    pub errors: Option<Vec<OutError<SourceLocation, ErrType>>>,
    pub sources: HashMap<String, HashMap<String, i32>>,
    // "sourceFile.sol" { "ContractName" { ... } }
    pub contracts: Option<HashMap<String, HashMap<String, SolcContract>>>,
}

impl From<SolcOut> for CompilerOutput {
    fn from(val: SolcOut) -> Self {
        CompilerOutput::Solc(val)
    }
}

impl TryFrom<CompilerOutput> for SolcOut {
    type Error = String; // TODO not very idiomatic

    fn try_from(value: CompilerOutput) -> Result<SolcOut, String> {
        match value {
            CompilerOutput::Solc(solc_out) => Ok(solc_out),
            _ => Err(String::from("Error with TryFrom !")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonError {
    pub message: String,
}

impl From<JsonError> for CompilerError {
    fn from(val: JsonError) -> Self {
        CompilerError::BinError(BinError::Json(val))
    }
}

impl Error for JsonError {}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn opt_none<T>(val: &Option<T>) -> bool {
    val.is_none()
}

impl RunCompiler for Solc {
    fn run(&self) -> Result<CompilerOutput, CompilerError> {
        let input = serde_json::to_string(&self).unwrap();

        let mut child = Command::new("solc")
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

        let solc_out = if let Ok(solc_out) = serde_json::from_str::<SolcOut>(&raw_out) {
            solc_out
        } else {
            panic!("failed to deserialize solc output: {}", &raw_out);
        };

        if let Some(errs) = &solc_out.errors {
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
                Ok(solc_out.into())
            }
        } else {
            Ok(solc_out.into())
        }
    }
}
