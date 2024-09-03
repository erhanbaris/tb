use tb_core::types::{Number, Value};

use crate::{instruction::X86Instruction, register::Register, X86AddressingMode, X86ApplicationContext, X86Location, X86Store};

use super::error::X86Error;

pub struct X86ValueCompiler;

impl X86ValueCompiler {
    pub fn compile(variable: Value, context: &mut X86ApplicationContext, scope: &mut X86Store, target: Option<X86Location>) -> Result<X86Location, X86Error> {
        match variable {
            Value::Variable(variable) => {
                let position = scope.find_variable(&variable).ok_or(X86Error::VariableNotFound(variable.to_owned()))?;
                
                if let Some(target) = target {
                    // Copy value from stack to new location
                    context.instructions.add_instruction(X86Instruction::Mov {
                        source: X86Location::Register(X86AddressingMode::create_based(position as i32 * -4, Register::RBP)),
                        target,
                        comment: None
                    });

                    // Return expected target
                    Ok(target)
                } else {
                    Ok(X86Location::Register(X86AddressingMode::create_based(position as i32 * -4, Register::RBP)))
                }
            },

            Value::Number(num) => {
                match target {
                    Some(target) => {
                        context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Imm(Number::I64(num)), target, comment: None });
                        Ok(target)
                    },
                    None => {
                        let register = scope.lock_register().ok_or(X86Error::NoRegisterAvailable)?;
                        context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Imm(Number::I64(num)), target: X86Location::Register(X86AddressingMode::Direct(register)), comment: None });
                        Ok(X86Location::Register(X86AddressingMode::Direct(register)))
                    }
                }
            },
        }
    }
}
