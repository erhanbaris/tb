use tb_core::types::Block;

use crate::{X86ApplicationContext, X86Store};

use super::{error::X86Error, X86StatementCompiler};

pub struct X86BlockCompiler;

impl X86BlockCompiler {
    pub fn compile(block: Block, scope: &mut X86Store, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        for item in block.items.into_iter() {
            X86StatementCompiler::compile(item, scope, context)?;
        }
        Ok(())
    }
}
