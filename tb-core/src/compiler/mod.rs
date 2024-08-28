use std::{collections::HashMap, fmt::Display, path::PathBuf};

use gcc::GCCCompiler;

use crate::error::TBError;

mod gcc;

#[derive(Debug)]
pub enum CompilerType {
    GCC
}

impl Display for CompilerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerType::GCC => write!(f, "GCC")
        }
    }
}

pub trait CompilerTrait where Self: Sized {
    fn create() -> Result<Self, TBError>;
    fn name(&self) -> &'static str;
    fn version(&self) -> &str;
    fn path(&self) -> &PathBuf;
    fn compile(&self, file_path: &str, target: &str, arguments: HashMap<String, String>) -> Result<(), TBError>;
}

pub struct TBCompiler;

impl TBCompiler {
    pub fn get_compiler() -> Result<Box<impl CompilerTrait>, TBError> {
        let compiler = GCCCompiler::create()?;

        log::info!("Compiler found: {}", compiler.name());
        log::info!("Version: {}", compiler.version());
        log::info!("Path: {}", compiler.path().display());

        Ok(Box::new(compiler))
    }
}