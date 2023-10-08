use super::builder::{BinError, CompilerError, CompilerOutput, RunCompiler};
use core::fmt;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    process::{Command, Stdio},
};

#[derive(Serialize)]
pub struct Source {
    pub content: String,
}

#[derive(Default, Serialize)]
pub struct SolcSources {
    pub sources: HashMap<String, Source>,
}

impl SolcSources {
    pub fn new(source_content: HashMap<String, String>) -> Self {
        let sources = source_content
            .into_iter()
            .map(|(file, content)| (file, Source { content }))
            .collect();
        SolcSources { sources }
    }
}

#[derive(Clone, Serialize)]
pub enum OutputOption {
    #[serde(rename = "lowercase")]
    Metadata,
    #[serde(rename = "evm.bytecode")]
    EvmBytecode,
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

// https://docs.soliditylang.org/en/latest/using-the-compiler.html#output-description
pub struct SolcOutput {}

impl From<SolcOutput> for CompilerOutput {
    fn from(val: SolcOutput) -> Self {
        CompilerOutput::Solc(val)
    }
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
pub struct SolcCodeError {
    #[serde(rename = "sourceLocation")]
    source_location: Option<SourceLocation>,
    #[serde(rename = "secondarySourceLocation")]
    secondary_source_locations: Option<Vec<SourceLocation>>,
    #[serde(rename = "type")]
    err_type: ErrType,
    component: String,
    severity: Severity,
    #[serde(rename = "errorCode")]
    error_code: Option<i32>,
    message: String,
    #[serde(rename = "formattedMessage")]
    formatted_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugData {
    #[serde(rename = "entryPoint")]
    entry_point: i32,
    id: Option<i32>,
    #[serde(rename = "parameterSlots")]
    parameter_slots: Option<i32>,
    #[serde(rename = "returnSlots")]
    return_slots: Option<i32>,
}

// TODO
type Ast = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedSource {
    ast: Ast,
    contents: String,
    id: i32,
    language: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BytecodeData {
    #[serde(rename = "functionDebugData")]
    function_debug_data: HashMap<String, DebugData>,
    object: String,
    opcodes: String,
    #[serde(rename = "sourceMap")]
    source_map: String,
    #[serde(rename = "generatedSources")]
    generated_sources: Vec<GeneratedSource>,
    #[serde(rename = "linkReferences")]
    link_references: HashMap<String, HashMap<String, Vec<SourceLocation>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedBytecodeData {
    #[serde(flatten)]
    bytecode_data: BytecodeData,
    #[serde(rename = "immutableReferences")]
    immutable_references: HashMap<String, String>, // TODO
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvmOutput {
    assembly: Option<String>,
    #[serde(rename = "legacyAssembly")]
    legacy_assembly: Option<HashMap<String, String>>,
    bytecode: Option<BytecodeData>,
    #[serde(rename = "deployedBytecode")]
    deployed_bytecode: Option<DeployedBytecodeData>,
    #[serde(rename = "methodIdentifiers")]
    method_identifiers: Option<HashMap<String, String>>,
    #[serde(rename = "gasEstimates")]
    gas_estimates: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLayout {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    abi: Option<Vec<String>>,
    metadata: Option<String>,
    #[serde(rename = "userdoc")]
    user_doc: Option<HashMap<String, String>>,
    #[serde(rename = "devdoc")]
    dev_doc: Option<HashMap<String, String>>,
    ir: Option<String>,
    #[serde(rename = "irAst")]
    ir_ast: Option<HashMap<String, String>>,
    #[serde(rename = "irOptimized")]
    ir_optimized: Option<String>,
    #[serde(rename = "irOptimizedAst")]
    ir_optimized_ast: Option<HashMap<String, String>>,
    #[serde(rename = "storageLayout")]
    storage_layout: Option<StorageLayout>,
    evm: Option<EvmOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolcOut {
    errors: Option<Vec<SolcCodeError>>,
    sources: HashMap<String, HashMap<String, i32>>,
    // "sourceFile.sol" { "ContractName" { ... } }
    contracts: Option<HashMap<String, HashMap<String, Contract>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolcError {
    pub message: String,
}

impl From<SolcError> for CompilerError {
    fn from(val: SolcError) -> Self {
        CompilerError::BinError(BinError::Solc(val))
    }
}

impl Error for SolcError {}

impl fmt::Display for SolcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn opt_false(b: &bool) -> bool {
    !(*b)
}

fn opt_none<T>(val: &Option<T>) -> bool {
    val.is_none()
}

impl RunCompiler for Solc {
    fn run(&self) -> Result<CompilerOutput, CompilerError> {
        let input = serde_json::to_string(&self).unwrap();
        eprintln!("{:?}", input);

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
        eprintln!("{:?}", raw_out);

        if !output.status.success() {
            return Err(SolcError {
                message: String::from_utf8(output.stderr).unwrap(),
            }
            .into());
        }

        let solc_out = if let Ok(solc_out) = serde_json::from_str::<SolcOut>(&raw_out) {
            solc_out
        } else {
            panic!("failed to deserialize solc output: {}", &raw_out);
        };
        dbg!(&solc_out);

        if let Some(errs) = solc_out.errors {
            if !errs.is_empty() {
                Err(SolcError {
                    message: errs.first().unwrap().message.clone(),
                }
                .into())
            } else {
                Ok(SolcOutput {}.into())
            }
        } else {
            Ok(SolcOutput {}.into())
        }
    }
}
