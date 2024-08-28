use tb_core::types::{Expression, Number, Statement, Value};

use crate::{register::Register, X86AddressingMode, instruction::X86Instruction, X86Location, X86Store};

use super::expression::X86ExpressionGenerator;

pub struct X86StatementGenerator;


impl X86StatementGenerator {
    pub fn generate(statement: Statement, scope: &mut X86Store) -> Vec<X86Instruction> {
        match statement {
            Statement::Assign { name, assigne } => Self::generate_assign(scope, name, assigne),
            Statement::Return(expr) => Self::generate_return(scope, expr),
        }
    }
    
    fn generate_assign(scope: &mut X86Store, name: String, assigne: Expression) -> Vec<X86Instruction> {
        let position = match scope.find_variable(&name) {
            Some(index) => index,
            None => scope.add_variable(&name)
        };

        let registers = scope.register_backup();

        let mut instructions = X86ExpressionGenerator::generate(assigne, scope);

        if let Some(mode) = scope.get_last_assigned_location().get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(X86Instruction::Mov { source: scope.get_last_assigned_location(), target: X86Location::Register(X86AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Immediate(new_reg)));
            }
        }

        instructions.push(X86Instruction::Mov { source: scope.get_last_assigned_location(), target: X86Location::Register(X86AddressingMode::Based(position as i32 * -4, Register::RBP)), comment: Some(format!("assign {}", name)) });
        scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Based(position as i32 * -4, Register::RBP)));
        scope.register_restore(registers);
        instructions
    }

    fn generate_return(scope: &mut X86Store, expr: Option<Value>) -> Vec<X86Instruction> {
        match expr {
            Some(Value::Variable(variable)) => {
                if let Some(_position) = scope.find_variable(&variable) {
                    return vec![X86Instruction::Mov { source: scope.get_last_assigned_location(), target: X86Location::Register(X86AddressingMode::Immediate(Register::RAX)), comment: Some(format!("return {}", variable)) }]
                }
                Vec::new()
            },
            Some(Value::Number(variable)) => vec![X86Instruction::Mov { source: X86Location::Imm(Number::I32(variable)), target: X86Location::Register(X86AddressingMode::Immediate(Register::RAX)), comment: Some(format!("return {}", variable)) }],
            None => Vec::default()
        }
    }
}