use tb_core::types::{Expression, Number, Statement, Value};

use crate::{backend::{Instruction, Location}, register::{AddressingMode, Register}, X86Store};

use super::expression::X86ExpressionGenerator;

pub struct X86StatementGenerator;


impl X86StatementGenerator {
    pub fn generate(statement: Statement, scope: &mut X86Store) -> Vec<Instruction> {
        match statement {
            Statement::Assign { name, assigne } => Self::generate_assign(scope, name, assigne),
            Statement::Return(expr) => Self::generate_return(scope, expr),
        }
    }
    
    fn generate_assign(scope: &mut X86Store, name: String, assigne: Expression) -> Vec<Instruction> {
        let position = match scope.find_variable(&name) {
            Some(index) => index,
            None => scope.add_variable(&name)
        };

        let registers = scope.register_backup();

        let mut instructions = X86ExpressionGenerator::generate(assigne, scope);

        if let Some(mode) = scope.get_last_assigned_location().get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(Instruction::Mov { source: scope.get_last_assigned_location(), target: Location::Register(AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                scope.set_last_assigned_location(Location::Register(AddressingMode::Immediate(new_reg)));
            }
        }

        instructions.push(Instruction::Mov { source: scope.get_last_assigned_location(), target: Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP)), comment: Some(format!("assign {}", name)) });
        scope.set_last_assigned_location(Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP)));
        scope.register_restore(registers);
        instructions
    }

    fn generate_return(scope: &mut X86Store, expr: Option<Value>) -> Vec<Instruction> {
        match expr {
            Some(Value::Variable(variable)) => {
                if let Some(position) = scope.find_variable(&variable) {
                    return vec![Instruction::Mov { source: scope.get_last_assigned_location(), target: Location::Register(AddressingMode::Immediate(Register::RAX)), comment: Some(format!("return {}", variable)) }]
                }
                Vec::new()
            },
            Some(Value::Number(variable)) => vec![Instruction::Mov { source: Location::Imm(Number::I32(variable)), target: Location::Register(AddressingMode::Immediate(Register::RAX)), comment: Some(format!("return {}", variable)) }],
            None => Vec::default()
        }
    }
}