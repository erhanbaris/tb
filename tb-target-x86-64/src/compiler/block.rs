use tb_core::types::Block;

use crate::{instruction::X86Instruction, X86ApplicationContext, X86Store};

use super::{error::X86Error, X86StatementCompiler};

pub struct X86BlockCompiler;

impl X86BlockCompiler {
    pub fn compile(block: Block, scope: &mut X86Store, context: &mut X86ApplicationContext) -> Result<Vec<X86Instruction>, X86Error> {
        let mut instructions: Vec<X86Instruction> = Vec::new();
        for item in block.items.into_iter() {
            instructions.append(&mut X86StatementCompiler::compile(item, scope, context)?);
        }
        Ok(instructions)
    }
}
