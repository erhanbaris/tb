use tb_core::types::{Condition, Value};

use crate::{instruction::{X86Instruction, X86InstructionType}, X86ApplicationContext, X86Store};

use super::{error::X86Error, X86ValueCompiler};

pub struct X86ConditionCompiler;


impl X86ConditionCompiler {
    pub fn compile(condition: Condition, scope: &mut X86Store, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        match condition {
            Condition::Eq { left, right } => Self::compile_simple(scope, X86InstructionType::Cmp, left, right, context),
        }
    }
    
    fn compile_simple(scope: &mut X86Store, inst_type: X86InstructionType, left: Value, right: Value, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let registers = scope.register_backup();

        context.instructions.add_comment("Generate left value".to_owned());
        let left = X86ValueCompiler::compile(left, context, scope, None)?;

        context.instructions.add_comment("Generate right value".to_owned());
        let right = X86ValueCompiler::compile(right, context, scope, None)?;

        let instruction = match inst_type {
            X86InstructionType::Cmp => X86Instruction::Cmp { left, right, comment: None },
            _ => return Err(X86Error::UnexpectedInstruction)
        };

        context.instructions.add_instruction(instruction);
        scope.register_restore(registers);

        Ok(())
    }
}
