use tb_core::types::{Number, Value};

use crate::{register::Register, X86AddressingMode, instruction::X86Instruction, X86Location, X86Store};

pub struct X86ValueCompiler;

impl X86ValueCompiler {
    pub fn compile(variable: Value, instructions: &mut Vec<X86Instruction>, scope: &mut X86Store, target: Option<X86Location>) -> X86Location {
        match variable {
            Value::Variable(variable) => match scope.find_variable(&variable) {
                Some(position) => {
                    match target {
                        Some(target) => {
                            // Copy value from stack to new location
                            instructions.push(X86Instruction::Mov {
                                source: X86Location::Register(X86AddressingMode::create_based(position as i32 * -4, Register::RBP)),
                                target: target.clone(),
                                comment: None
                            });

                            // Return expected target
                            target
                        },
                        None => X86Location::Register(X86AddressingMode::create_based(position as i32 * -4, Register::RBP))
                    }
                },
                None => panic!("variable not found")
            },
            Value::Number(num) => {
                match target {
                    Some(target) => {
                        instructions.push(X86Instruction::Mov { source: X86Location::Imm(Number::I32(num)), target: target.clone(), comment: None });
                        target
                    },
                    None => {
                        let register = scope.lock_register().unwrap();
                        instructions.push(X86Instruction::Mov { source: X86Location::Imm(Number::I32(num)), target: X86Location::Register(X86AddressingMode::Direct(register)), comment: None });
                        X86Location::Register(X86AddressingMode::Direct(register))
                    }
                }
            },
        }
    }
}
