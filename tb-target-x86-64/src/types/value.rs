use tb_core::types::{Number, Value};

use crate::{register::Register, X86AddressingMode, instruction::X86Instruction, X86Location, X86Store};

pub struct X86ValueGenerator;

impl X86ValueGenerator {
    pub fn generate(variable: Value, instructions: &mut Vec<X86Instruction>, scope: &mut X86Store) -> X86Location {
        match variable {
            Value::Variable(variable) => match scope.find_variable(&variable) {
                Some(position) => X86Location::Register(X86AddressingMode::create_based(position as i32 * -4, Register::RBP)),
                None => panic!("variable not found")
            },
            Value::Number(num) => {
                let position = scope.add_temp_variable();
                let stack = X86Location::Register(X86AddressingMode::Based(position as i32 * -4, Register::RBP));

                instructions.push(X86Instruction::Mov { source: X86Location::Imm(Number::I32(num)), target: stack, comment: None });

                let register = scope.lock_register().unwrap();
                instructions.push(X86Instruction::Mov { source: stack, target: X86Location::Register(X86AddressingMode::Immediate(register)), comment: None });
                X86Location::Register(X86AddressingMode::Immediate(register))
            },
        }
    }
}
