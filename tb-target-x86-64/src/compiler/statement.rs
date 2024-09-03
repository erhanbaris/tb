use tb_core::types::{Block, Condition, Expression, Number, Statement, Value};

use crate::{instruction::X86Instruction, register::Register, X86AddressingMode, X86ApplicationContext, X86Location, X86Store};

use super::{block::X86BlockCompiler, condition::X86ConditionCompiler, error::X86Error, expression::X86ExpressionCompiler};

pub struct X86StatementCompiler;


impl X86StatementCompiler {
    pub fn compile(statement: Statement, scope: &mut X86Store, context: &mut X86ApplicationContext) -> Result<Vec<X86Instruction>, X86Error> {
        match statement {
            Statement::Assign { name, assigne } => Self::compile_assign(scope, name, assigne),
            Statement::Return(expr) => Self::compile_return(scope, expr),
            Statement::If { condition, true_block, false_block } => Self::compile_if(condition, true_block, false_block, context),
        }
    }
    
    fn compile_assign(scope: &mut X86Store, name: String, assigne: Expression) -> Result<Vec<X86Instruction>, X86Error> {
        let position = match scope.find_variable(&name) {
            Some(index) => index,
            None => scope.add_variable(&name)
        };

        let registers = scope.register_backup();

        let mut instructions = X86ExpressionCompiler::compile(assigne, scope)?;

        if let Some(mode) = scope.get_last_assigned_location().get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().ok_or(X86Error::NoRegisterAvailable)?;
                instructions.push(X86Instruction::Mov { source: scope.get_last_assigned_location(), target: X86Location::Register(X86AddressingMode::Direct(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Direct(new_reg)));
            }
        }

        instructions.push(X86Instruction::Mov { source: scope.get_last_assigned_location(), target: X86Location::Register(X86AddressingMode::Based(position as i32 * -4, Register::RBP)), comment: Some(format!("assign {}", name)) });
        scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Based(position as i32 * -4, Register::RBP)));
        scope.register_restore(registers);
        Ok(instructions)
    }

    fn compile_if(condition: Condition, true_block: Block, false_block: Option<Block>, context: &mut X86ApplicationContext) -> Result<Vec<X86Instruction>, X86Error> {
        let mut scope = X86Store::default();
        let mut instructions = Vec::new();

        let condition_instructions = X86ConditionCompiler::compile(condition, &mut scope)?;
        instructions.extend(condition_instructions);

        let true_instructions = X86BlockCompiler::compile(true_block, &mut scope, context)?;
        instructions.extend(true_instructions);

        if let Some(false_block) = false_block {
            //let else_branch = context.storage.get_branch();
            let false_instructions = X86BlockCompiler::compile(false_block, &mut scope, context)?;
            instructions.extend(false_instructions);
        };

        Ok(instructions)
    }

    fn compile_return(scope: &mut X86Store, expr: Option<Value>) -> Result<Vec<X86Instruction>, X86Error> {
        let instructions = match expr {
            Some(Value::Variable(variable)) => {
                match scope.find_variable(&variable) {
                    Some(position) => vec![X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Based(position as i32 * -4, Register::RBP)), target: X86Location::Register(X86AddressingMode::Direct(Register::EAX)), comment: Some(format!("return {}", variable)) }],
                    None => Vec::new()
                }
            },
            Some(Value::Number(variable)) => vec![X86Instruction::Mov { source: X86Location::Imm(Number::I64(variable)), target: X86Location::Register(X86AddressingMode::Direct(Register::EAX)), comment: Some(format!("return {}", variable)) }],
            None => Vec::default()
        };

        Ok(instructions)
    }
}