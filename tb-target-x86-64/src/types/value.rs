use tb_core::types::{Number, Value};

use crate::{backend::{Instruction, Location}, register::{AddressingMode, Register}, X86Store};

pub struct X86ValueGenerator;

impl X86ValueGenerator {
    pub fn generate(variable: Value, instructions: &mut Vec<Instruction>, scope: &mut X86Store) -> Location {
        match variable {
            Value::Variable(variable) => match scope.find_variable(&variable) {
                Some(position) => Location::Register(AddressingMode::create_based(position as i32 * -4, Register::RBP)),
                None => panic!("variable not found")
            },
            Value::Number(num) => {
                let position = scope.add_temp_variable();
                let stack = Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP));

                instructions.push(Instruction::Mov { source: Location::Imm(Number::I32(num)), target: stack, comment: None });

                let register = scope.lock_register().unwrap();
                instructions.push(Instruction::Mov { source: stack, target: Location::Register(AddressingMode::Immediate(register)), comment: None });
                Location::Register(AddressingMode::Immediate(register))
            },
        }
    }
}
