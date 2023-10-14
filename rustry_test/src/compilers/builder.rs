use crate::compilers::{
    huff::huffc::HuffcBuilder,
    solidity::solc::{
        JsonError, OutputOption, Settings, SolcBuilder, SolcBuilderError, SolcOut, Source,
    },
};
use core::fmt;
use std::{collections::HashMap, fs::File, io::Write};
use tempfile::tempdir;

use super::{
    huff::huffc::HuffcOut,
    vyper::vyperc::{self, VypercBuilder, VypercOut},
};

#[derive(Debug)]
pub enum CompilerKinds {
    Solc,
    Vyper,
    Huff,
}

#[derive(Debug)]
pub struct Compiler {
    pub sources: HashMap<String, String>,
    pub kind: CompilerKinds,
}

#[derive(Debug)]
pub enum CompilerOutput {
    Solc(SolcOut),
    Vyper(VypercOut),
    Huff(HuffcOut),
}

#[derive(Debug)]
pub enum BuilderError {
    Solc(SolcBuilderError),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Solc(e) => write!(f, "solc builder error: {e}"),
        }
    }
}

impl fmt::Display for BinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(e) => write!(f, "compiler bin error: {e}"),
        }
    }
}

#[derive(Debug)]
pub enum BinError {
    Json(JsonError),
}

#[derive(Debug)]
pub enum CompilerError {
    BuilderError(BuilderError),
    BinError(BinError),
}

impl From<BinError> for CompilerError {
    fn from(e: BinError) -> Self {
        Self::BinError(e)
    }
}

impl From<BuilderError> for CompilerError {
    fn from(e: BuilderError) -> Self {
        Self::BuilderError(e)
    }
}

impl std::error::Error for CompilerError {}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::BuilderError(e) => write!(f, "builder error: {e}"),
            Self::BinError(e) => write!(f, "bin error: {e}"),
        }
    }
}

pub trait RunCompiler {
    fn run(&self) -> Result<CompilerOutput, CompilerError>;
}

impl Compiler {
    pub fn run(&self) -> Result<CompilerOutput, CompilerError> {
        match self.kind {
            CompilerKinds::Solc => {
                let mut solc = SolcBuilder::default()
                    .bin(true)
                    .settings(Some(Settings {
                        remappings: None,
                        output_selection: HashMap::from([(
                            String::from("*"),
                            HashMap::from([(
                                String::from("*"),
                                vec![
                                    OutputOption::EvmBytecode,
                                    OutputOption::Metadata,
                                    OutputOption::Abi,
                                ],
                            )]),
                        )]),
                    }))
                    .build()
                    .unwrap();
                solc.sources = self
                    .sources
                    .clone()
                    .into_iter()
                    .map(|(file, content)| (file, Source { content }))
                    .collect();

                solc.run()
            }
            CompilerKinds::Huff => {
                let dir = tempdir().unwrap(); // deleted when the destructor is ran
                let input = dir.path().join("input.huff");
                let input = input.into_os_string().into_string().unwrap();
                let mut file = File::create(input.clone()).unwrap();
                let content = self.sources.iter().next().unwrap().1;
                writeln!(file, "{}", content).unwrap();
                let output = dir.path().join("output.json");
                File::create(output.clone()).unwrap();
                let output = output.into_os_string().into_string().unwrap();

                let mut huffc = HuffcBuilder::default()
                    .artifacts(true)
                    .input(input)
                    .output(output)
                    .build()
                    .unwrap();
                huffc.attach_dir(dir);

                huffc.run()
            }
            CompilerKinds::Vyper => {
                let mut vyc = VypercBuilder::default()
                    .settings(Some(super::vyper::vyperc::Settings {
                        evm_version: String::from("paris"),
                        output_selection: HashMap::from([(
                            String::from("*"),
                            HashMap::from([(
                                String::from("*"),
                                vec![vyperc::OutputOption::EvmBytecode, vyperc::OutputOption::Abi],
                            )]),
                        )]),
                    }))
                    .build()
                    .unwrap();
                vyc.sources = self
                    .sources
                    .clone()
                    .into_iter()
                    .map(|(file, content)| (file, Source { content }))
                    .collect();

                vyc.run()
            }
        }
    }
}
