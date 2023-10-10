use std::{
    fs::{self, File},
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{
    common::utils::opt_false,
    compilers::builder::{CompilerError, CompilerOutput, RunCompiler},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
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
    path: String,
    source: String,
    access: Option<String>,    // TODO
    dependencies: Vec<String>, // TODO
}

#[derive(Debug, Deserialize)]
pub struct Functions {}

#[derive(Debug, Deserialize)]
pub struct Events {}

#[derive(Debug, Deserialize)]
pub struct Errors {}

#[derive(Debug, Deserialize)]
pub struct HuffAbi {
    constructor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<Functions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<Events>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Errors>,
    receive: bool,
    fallback: bool,
}

#[derive(Debug, Deserialize)]
pub struct HuffOut {
    file: HuffFile,
    bytecode: String,
    runtime: String,
    abi: HuffAbi,
}

impl From<HuffOut> for CompilerOutput {
    fn from(val: HuffOut) -> Self {
        CompilerOutput::Huff(val)
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
        println!("{}", serde_json::to_string_pretty(&out_content).unwrap());
        let huffc_out = if let Ok(huffc_out) = serde_json::from_str::<HuffOut>(&out_content) {
            huffc_out
        } else {
            panic!("failed to deserialize huffc output: {}", &out_content);
        };

        Ok(huffc_out.into())
    }
}
