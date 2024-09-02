use std::{collections::HashMap, path::PathBuf, process::Command, str};

use crate::error::TBError;

use super::{CompilerTrait, CompilerType};
use which::which;

#[derive(Debug, Clone, Default)]
pub struct GCCCompiler {
    version: String,
    path: PathBuf
}

impl CompilerTrait for GCCCompiler {
    fn create() -> Result<Self, TBError> {
        let mut compiler = GCCCompiler::default();
        let compiler_path = which("gcc").map_err(|_| TBError::CompilerNotFound(CompilerType::GCC))?;

        let result = Command::new(&compiler_path).arg("--version").output()?;
        let version = String::from_utf8(result.stdout)?;
        
        compiler.version = version.lines().nth(0).unwrap_or_default().to_string();
        compiler.path = compiler_path;

        Ok(compiler)
    }

    fn name(&self) -> &'static str {
        "GCC"
    }
    
    fn version(&self) -> &str {
        &self.version
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }
    
    fn compile(&self, file_path: &PathBuf, target: &PathBuf, arguments: HashMap<String, String>) -> Result<(), TBError> {
        let mut command = Command::new(&self.path);
        let mut args = command
            .arg(file_path)
            .arg("-o")
            .arg(target);
        for (arg1, arg2) in arguments.into_iter() {
            args = args.arg(arg1).arg(arg2);
        }

        let result = args.output()?;

        if result.status.success() {
            log::info!("Compile operation successfully completed.");
            log::info!("Executable location: {}", &target.display());
            return Ok(());
        }

        let error_message =  String::from_utf8(result.stderr)?;

        log::error!("Compile operation failed.");
        log::error!("{}", &error_message);

        Err(TBError::CompileFailed(error_message))
    }
}
