use tb_core::types::{Expression, Value};

use crate::{X86AddressingMode, instruction::X86Instruction, X86Location, X86Store};

use super::value::X86ValueCompiler;

pub struct X86ExpressionCompiler;

impl X86ExpressionCompiler {
    pub fn compile(expression: Expression, scope: &mut X86Store) -> Vec<X86Instruction> {
        match expression {
            Expression::Add { target, source } => Self::compile_add(scope, target, source),
            Expression::Sub { target, source } => Self::compile_sub(scope, target, source),
            Expression::Not { source } => Self::compile_not(scope, source),
            Expression::Neg { source } => Self::compile_neg(scope, source),
            Expression::Value(val) => Self::compile_value(val),
        }
    }

    fn compile_add(scope: &mut X86Store, target: Value, source: Value) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate source value".to_owned()));
        let mut source = X86ValueCompiler::compile(source, &mut instructions, scope);

        instructions.push(X86Instruction::Comment("Generate target value".to_owned()));
        let target = X86ValueCompiler::compile(target, &mut instructions, scope);

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(X86Instruction::Mov { source, target: X86Location::Register(X86AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = X86Location::Register(X86AddressingMode::Immediate(new_reg));
            }
        }

        instructions.push(X86Instruction::Add { source, target, comment: None });
        scope.register_restore(registers);
        scope.set_last_assigned_location(target);

        if let Some(register) = target.get_register() {
            scope.mark_register(register);
        }

        instructions
    }

    fn compile_sub(scope: &mut X86Store, target: Value, source: Value) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate source value".to_owned()));
        let mut source = X86ValueCompiler::compile(source, &mut instructions, scope);

        instructions.push(X86Instruction::Comment("Generate target value".to_owned()));
        let target = X86ValueCompiler::compile(target, &mut instructions, scope);

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(X86Instruction::Mov { source, target: X86Location::Register(X86AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = X86Location::Register(X86AddressingMode::Immediate(new_reg));
            }
        }

        instructions.push(X86Instruction::Sub { source, target, comment: None });
        scope.register_restore(registers);
        scope.set_last_assigned_location(target);

        if let Some(register) = target.get_register() {
            scope.mark_register(register);
        }

        instructions
    }

    fn compile_not(scope: &mut X86Store, source: Value) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate source value".to_owned()));
        let mut source = X86ValueCompiler::compile(source, &mut instructions, scope);

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(X86Instruction::Mov { source, target: X86Location::Register(X86AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = X86Location::Register(X86AddressingMode::Immediate(new_reg));
            }
        }

        instructions.push(X86Instruction::Not { source, comment: None });
        scope.register_restore(registers);
        scope.set_last_assigned_location(source);

        if let Some(register) = source.get_register() {
            scope.mark_register(register);
        }

        instructions
    }

    fn compile_neg(scope: &mut X86Store, source: Value) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate source value".to_owned()));
        let mut source = X86ValueCompiler::compile(source, &mut instructions, scope);

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(X86Instruction::Mov { source, target: X86Location::Register(X86AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = X86Location::Register(X86AddressingMode::Immediate(new_reg));
            }
        }

        instructions.push(X86Instruction::Neg { source, comment: None });
        scope.register_restore(registers);
        scope.set_last_assigned_location(source);

        if let Some(register) = source.get_register() {
            scope.mark_register(register);
        }

        instructions
    }

    pub fn compile_value(value: Value) -> Vec<X86Instruction> {
        match value {
            Value::Variable(_) => todo!(),
            Value::Number(_) => todo!(),
        }
    }
}
