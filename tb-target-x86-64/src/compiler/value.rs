use tb_core::types::Value;

use crate::{instruction::X86Instruction, register::Register, X86AddressingMode, X86ApplicationContext, X86Location, X86Store};

use super::error::X86Error;

pub struct X86ValueCompiler;

impl X86ValueCompiler {
    pub fn compile(variable: Value, context: &mut X86ApplicationContext, scope: &mut X86Store, target: Option<X86Location>) -> Result<X86Location, X86Error> {
        match variable {
            Value::Variable(variable) => {
                let (variable_size, variable_position) = {
                    let variable = scope.find_variable(&variable).ok_or(X86Error::VariableNotFound(variable.to_owned()))?;
                    (variable.size, variable.position)
                };

                scope.set_last_size(variable_size.into());

                if let Some(target) = target {
                    // Copy value from stack to new location
                    context.instructions.add_instruction(X86Instruction::Mov {
                        source: X86Location::Register(X86AddressingMode::create_based(-(variable_position as i32), Register::RBP)),
                        target: target.clone(),
                        comment: None
                    });

                    // Return expected target
                    Ok(target)
                } else {
                    Ok(X86Location::Register(X86AddressingMode::create_based(-(variable_position as i32), Register::RBP)))
                }
            },

            Value::Number(num) => {
                let num_size = num.size();
                scope.set_last_size(num_size);

                match target {
                    Some(target) => {
                        context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Imm(num), target: target.clone(), comment: None });
                        Ok(target)
                    },
                    None => {
                        let register = scope.lock_register(num_size).ok_or(X86Error::NoRegisterAvailable)?;
                        context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Imm(num), target: X86Location::Register(X86AddressingMode::Direct(register)), comment: None });
                        Ok(X86Location::Register(X86AddressingMode::Direct(register)))
                    }
                }
            },
        }
    }
}
