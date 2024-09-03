use tb_core::types::{Condition, Value};

use crate::{instruction::{X86Instruction, X86InstructionType}, X86Store};

use super::{error::X86Error, X86ValueCompiler};

pub struct X86ConditionCompiler;


impl X86ConditionCompiler {
    pub fn compile(condition: Condition, scope: &mut X86Store) -> Result<Vec<X86Instruction>, X86Error> {
        match condition {
            Condition::Eq { left, right } => Self::compile_simple(scope, X86InstructionType::Cmp, left, right),
        }
    }
    
    fn compile_simple(scope: &mut X86Store, inst_type: X86InstructionType, left: Value, right: Value) -> Result<Vec<X86Instruction>, X86Error> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate left value".to_owned()));
        let left = X86ValueCompiler::compile(left, &mut instructions, scope, None)?;

        instructions.push(X86Instruction::Comment("Generate right value".to_owned()));
        let right = X86ValueCompiler::compile(right, &mut instructions, scope, None)?;

        let instruction = match inst_type {
            X86InstructionType::Cmp => X86Instruction::Cmp { left, right, comment: None },
            _ => return Err(X86Error::UnexpectedInstruction)
        };

        instructions.push(instruction);
        scope.register_restore(registers);

        Ok(instructions)
    }
}
