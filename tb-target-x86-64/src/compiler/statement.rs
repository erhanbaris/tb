use std::sync::LazyLock;

use tb_core::{location::Location, types::{Block, CallingConventions, Condition, ConditionDiscriminant, Expression, ProcedureCall, RegisterSize, Statement, Value}};

use crate::{instruction::X86Instruction, register::Register, X86AddressingMode, X86ApplicationContext, X86Location, X86Store};

use super::{block::X86BlockCompiler, condition::X86ConditionCompiler, error::X86Error, expression::X86ExpressionCompiler, X86ValueCompiler};

pub static CALL_CONVENTION: LazyLock<ProcedureCall<Register>>= LazyLock::new(|| {
    ProcedureCall {
        convention: CallingConventions::Systemv,
        registers: vec![Register::RDI, Register::RSI, Register::RDX, Register::RCX, Register::R8, Register::R9]
    }
});

pub struct X86StatementCompiler;


impl X86StatementCompiler {
    pub fn compile(statement: Statement, scope: &mut X86Store, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        match statement {
            Statement::Assign { name, assigne } => Self::compile_assign(scope, name, assigne, context),
            Statement::Call { name, arguments, assign } => Self::compile_call(scope, name, arguments, assign, context),
            Statement::Print { format, argument } => Self::compile_print(scope, format, argument, context),
            Statement::Return(expr) => Self::compile_return(scope, expr, context),
            Statement::If { condition, true_block, false_block } => Self::compile_if(scope, condition, true_block, false_block, context),
        }
    }

    fn get_jump_instruction(condition_type: ConditionDiscriminant, label: &str) -> X86Instruction {
        match condition_type {
            ConditionDiscriminant::Eq => X86Instruction::Jne(label.to_owned()),
            ConditionDiscriminant::Ne => X86Instruction::Je(label.to_owned()),
            ConditionDiscriminant::Gr => X86Instruction::Jnb(label.to_owned()),
            ConditionDiscriminant::Ge => X86Instruction::Jnbe(label.to_owned()),
            ConditionDiscriminant::Ls => X86Instruction::Jna(label.to_owned()),
            ConditionDiscriminant::Le => X86Instruction::Jnae(label.to_owned()),
        }
    }
    
    fn compile_assign(scope: &mut X86Store, name: String, assigne: Expression, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let position = match scope.find_variable(&name) {
            Some(variable) => variable.position,
            None => scope.add_variable(&name, 8).position
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
    
    fn compile_print(scope: &mut X86Store, format: String, argument: Option<Value>, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let mut arguments = Vec::new();
        arguments.push(Value::String(format));

        if let Some(argument) = argument {
            arguments.push(argument);
        }

        Self::compile_call(scope, context.os_specific_defs.print().to_owned(), arguments, None, context)
    }

    fn compile_call(scope: &mut X86Store, name: String, arguments: Vec<Value>, assign: Option<String>, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let registers = scope.register_backup();
        
        for (index, argument) in arguments.into_iter().enumerate().rev() {
            let register = (*CALL_CONVENTION).get_register(index);
            match register {
                Some(reg) => {
                    X86ValueCompiler::compile(argument.clone(), context, scope, Some(X86Location::Register(X86AddressingMode::Direct(reg))))?;
                },
                None => {
                    match argument {
                        Value::Variable(variable) => {
                            let variable_position = {
                                let variable = scope.find_variable(&variable).ok_or(X86Error::VariableNotFound(variable.to_owned()))?;
                                variable.position
                            };
                            context.instructions.add_instruction(X86Instruction::Push(X86Location::Register(X86AddressingMode::create_based(-(variable_position as i32), Register::RBP))));
                        },
                        Value::Number(num) => {
                            context.instructions.add_instruction(X86Instruction::Push(X86Location::Imm(num)));
                        },
                        Value::String(string) => {
                            let label = context.datas.create_label();
                            context.datas.add_string_data(&label, &string);
                            let tmp_register = scope.lock_register(RegisterSize::_64Bit).ok_or(X86Error::NoRegisterAvailable)?;
                            let variable = scope.add_temp_variable(8);
                            context.instructions.add_instruction(X86Instruction::Lea { source: X86Location::Label(label), target: X86Location::Register(X86AddressingMode::Direct(tmp_register)), comment: None });
                            context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Direct(tmp_register)), target: X86Location::Register(X86AddressingMode::Based(-(variable.position as i32), Register::RBP)), comment: None });
                            scope.release_register(tmp_register);
                        }
                    };
                },
            };
        }

        context.instructions.add_instruction(X86Instruction::Call(name));
        scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Direct(Register::RAX))); // call result is in RAX register

        if let Some(assigned) = assign {
            let position = match scope.find_variable(&assigned) {
                Some(variable) => variable.position,
                None => scope.add_variable(&assigned, 8).position
            };
            context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Direct(Register::RAX)), target: X86Location::Register(X86AddressingMode::Based(-(position as i32), Register::RBP)), comment: None });
        }
        scope.register_restore(registers);
        Ok(())
    }

    fn compile_if(scope: &mut X86Store, condition: Condition, true_block: Block, false_block: Option<Block>, context: &mut X86ApplicationContext) -> Result<(), X86Error> {        
        let condition_type = X86ConditionCompiler::compile(condition, scope, context)?;

        let fist_jump_location = context.instructions.add_instruction(Self::get_jump_instruction(condition_type, ""));

        X86BlockCompiler::compile(true_block, scope, context)?;

        if let Some(false_block) = false_block {
            let jmp_location = context.instructions.add_instruction(X86Instruction::Jne("".to_owned()));

            let else_branch = context.storage.create_branch();
            context.instructions.add_branch(else_branch.clone());
            X86BlockCompiler::compile(false_block, scope, context)?;

            let end_branch = context.storage.create_branch();
            context.instructions.add_branch(end_branch.clone());
            context.instructions.update_instruction(Self::get_jump_instruction(condition_type, &else_branch), fist_jump_location); // Jump to else block
            context.instructions.update_instruction(X86Instruction::Jmp(end_branch.to_owned()), jmp_location);

        } else {
            let end_branch = context.storage.create_branch();
            context.instructions.add_branch(end_branch.clone());
            context.instructions.update_instruction(Self::get_jump_instruction(condition_type, &end_branch), fist_jump_location);
        }

        Ok(())
    }

    fn compile_return(scope: &mut X86Store, expr: Option<Value>, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        match expr {
            Some(Value::Variable(variable)) => {
                if let Some(variable) = scope.find_variable(&variable) {
                    context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Based(-(variable.position as i32), Register::RBP)), target: X86Location::Register(X86AddressingMode::Direct(Register::EAX)), comment: Some(format!("return {}", variable.name)) });
                } else {
                    return Err(X86Error::VariableNotFound(variable.clone()));
                }
            },
            Some(Value::Number(number)) => {
                context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Imm(number), target: X86Location::Register(X86AddressingMode::Direct(Register::RAX)), comment: Some(format!("return {}", number)) });
            }
            Some(Value::String(data)) => {
                let label = context.datas.create_label();
                context.datas.add_string_data(&label, &data);
                context.instructions.add_instruction(X86Instruction::Lea { source: Location::Label(label), target: X86Location::Register(X86AddressingMode::Direct(Register::RAX)), comment: Some(format!("return \"{}\"", data)) });
                
            }
            None => ()
        };

        Ok(())
    }
}