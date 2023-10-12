use crate::{
    common::utils::opt_false,
    compilers::builder::{CompilerError, CompilerOutput, RunCompiler},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    process::{Command, Stdio},
};
use tempfile::TempDir;

#[derive(Builder, Serialize)]
pub struct Huffc {
    #[serde(skip_serializing_if = "opt_false")]
    #[serde(rename(serialize = "-a"))]
    pub artifacts: bool,
    // #[serde(skip_serializing_if = "String::default()")]
    #[serde(rename(serialize = "-o"))]
    pub output: String,
    pub input: String,
    #[builder(setter(skip))]
    #[serde(skip)]
    pub dir: Option<TempDir>,
}

impl Huffc {
    pub fn attach_dir(&mut self, dir: TempDir) {
        self.dir = Some(dir);
    }
}

#[derive(Debug, Deserialize)]
pub struct HuffFile {
    pub path: String,
    pub source: String,
    pub access: Option<String>,    // TODO
    pub dependencies: Vec<String>, // TODO
}

#[derive(Debug, Deserialize)]
pub struct Functions {}

#[derive(Debug, Deserialize)]
pub struct Events {}

#[derive(Debug, Deserialize)]
pub struct Errors {}

#[derive(Debug, Deserialize)]
pub struct HuffAbi {
    pub constructor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Functions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Events>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Errors>,
    pub receive: bool,
    pub fallback: bool,
}

#[derive(Debug, Deserialize)]
pub struct HuffcOut {
    pub file: HuffFile,
    pub bytecode: String,
    pub runtime: String,
    pub abi: HuffAbi,
}

impl From<HuffcOut> for CompilerOutput {
    fn from(val: HuffcOut) -> Self {
        CompilerOutput::Huff(val)
    }
}

impl TryFrom<CompilerOutput> for HuffcOut {
    type Error = String; // TODO not very idiomatic

    fn try_from(value: CompilerOutput) -> Result<HuffcOut, String> {
        match value {
            CompilerOutput::Huff(huffc_out) => Ok(huffc_out),
            _ => Err(String::from("Error with TryFrom !")),
        }
    }
}

impl RunCompiler for Huffc {
    fn run(&self) -> Result<CompilerOutput, CompilerError> {
        // TODO pipe stderr and return it
        let child = Command::new("huffc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .arg("-a")
            .args(["-o", &self.output])
            .arg(&self.input)
            .spawn()
            .unwrap();

        let output = child.wait_with_output().unwrap();
        if !output.status.success() {
            panic!("{} ", String::from_utf8(output.stderr).unwrap());
        }

        let out_content = fs::read_to_string(&self.output).unwrap();
        let huffc_out = if let Ok(huffc_out) = serde_json::from_str::<HuffcOut>(&out_content) {
            huffc_out
        } else {
            panic!("failed to deserialize huffc output: {}", &out_content);
        };

        Ok(huffc_out.into())
    }
}
