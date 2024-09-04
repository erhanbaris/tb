use tb_core::types::{Block, Condition, Expression, Statement, Value};

use crate::{instruction::X86Instruction, register::Register, X86AddressingMode, X86ApplicationContext, X86Location, X86Store};

use super::{block::X86BlockCompiler, condition::X86ConditionCompiler, error::X86Error, expression::X86ExpressionCompiler};

pub struct X86StatementCompiler;


impl X86StatementCompiler {
    pub fn compile(statement: Statement, scope: &mut X86Store, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        match statement {
            Statement::Assign { name, assigne } => Self::compile_assign(scope, name, assigne, context),
            Statement::Return(expr) => Self::compile_return(scope, expr, context),
            Statement::If { condition, true_block, false_block } => Self::compile_if(condition, true_block, false_block, context),
        }
    }
    
    fn compile_assign(scope: &mut X86Store, name: String, assigne: Expression, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let position = match scope.find_variable(&name) {
            Some(variable) => variable.position,
            None => scope.add_variable(&name, 4).position
        };

        let registers = scope.register_backup();

        X86ExpressionCompiler::compile(assigne, scope, context)?;

        if let Some(mode) = scope.get_last_assigned_location().get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register(scope.get_last_size()).ok_or(X86Error::NoRegisterAvailable)?;
                context.instructions.add_instruction(X86Instruction::Mov { source: scope.get_last_assigned_location(), target: X86Location::Register(X86AddressingMode::Direct(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Direct(new_reg)));
            }
        }

        context.instructions.add_instruction(X86Instruction::Mov { source: scope.get_last_assigned_location(), target: X86Location::Register(X86AddressingMode::Based(-(position as i32), Register::RBP)), comment: Some(format!("assign {}", name)) });
        scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Based(-(position as i32), Register::RBP)));
        scope.register_restore(registers);
        Ok(())
    }

    fn compile_if(condition: Condition, true_block: Block, false_block: Option<Block>, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let mut scope = X86Store::default();
        
        X86ConditionCompiler::compile(condition, &mut scope, context)?;
        X86BlockCompiler::compile(true_block, &mut scope, context)?;

        if let Some(false_block) = false_block {
            let else_branch = context.storage.create_branch();
            context.instructions.add_branch(else_branch);
            X86BlockCompiler::compile(false_block, &mut scope, context)?;
        };

        Ok(())
    }

    fn compile_return(scope: &mut X86Store, expr: Option<Value>, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        match expr {
            Some(Value::Variable(variable)) => {
                if let Some(variable) = scope.find_variable(&variable) {
                    context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Based(-(variable.position as i32), Register::RBP)), target: X86Location::Register(X86AddressingMode::Direct(Register::EAX)), comment: Some(format!("return {}", variable.name)) });
                }
            },
            Some(Value::Number(number)) => {
                context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Imm(number), target: X86Location::Register(X86AddressingMode::Direct(Register::EAX)), comment: Some(format!("return {}", number)) });
            }
            None => ()
        };

        Ok(())
    }
}