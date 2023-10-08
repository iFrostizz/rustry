use super::solidity::{
    self, OutputOption, Settings, SolcBuilder, SolcBuilderError, SolcError, SolcSources, Source,
};
use core::fmt;
use std::collections::HashMap;

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
    Solc(solidity::SolcOut),
    Vyper,
    Huff,
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
            Self::Solc(e) => write!(f, "solc bin error: {e}"),
        }
    }
}

#[derive(Debug)]
pub enum BinError {
    Solc(SolcError),
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
            CompilerKinds::Vyper => todo!(),
            CompilerKinds::Huff => todo!(),
        }
    }
}
