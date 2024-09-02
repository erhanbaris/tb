use std::string::FromUtf8Error;

use thiserror::Error;

use crate::compiler::CompilerType;

#[derive(Error, Debug)]
pub enum TBError {
    #[error("IO Error ({0})")]
    IOError(#[from] std::io::Error),

    #[error("Compiler not found ({0})")]
    CompilerNotFound(CompilerType),

    #[error("Unsupported string format ({0})")]
    UnsupportedStringFormat(#[from] FromUtf8Error),

    #[error("Compile failed ({0})")]
    CompileFailed(String)
}
