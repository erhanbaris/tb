use tb_core::{addressing_mode::AddressingMode, types::{Expression, Value}};

use crate::{instruction::{InstructionType, X86Instruction}, register::Register, X86AddressingMode, X86Location, X86Store};

use super::{error::X86Error, value::X86ValueCompiler};

const TARGET_ALWAYS_REGISTER: [InstructionType; 1] = [InstructionType::IMul];
pub struct X86ExpressionCompiler;

impl X86ExpressionCompiler {
    pub fn compile(expression: Expression, scope: &mut X86Store) -> Result<Vec<X86Instruction>, X86Error> {
        match expression {
            Expression::Add { target, source } => Self::compile_simple(scope, InstructionType::Add, target, source),
            Expression::Sub { target, source } => Self::compile_simple(scope, InstructionType::Sub, target, source),
            Expression::Mul { target, source } => Self::compile_simple(scope, InstructionType::IMul, target, source),
            Expression::Modulo { divider, divided } => Self::compile_div(scope, divider, divided, Register::EDX),
            Expression::Div { divider, divided } => Self::compile_div(scope, divider, divided, Register::EAX),
            Expression::BitwiseNot { source } => Self::compile_single(scope, InstructionType::Not, source),
            Expression::BitwiseAnd { source, target } => Self::compile_simple(scope, InstructionType::And, target, source),
            Expression::Neg { source } => Self::compile_single(scope, InstructionType::Neg, source),
            Expression::Value(val) => Self::compile_value(scope, val),
        }
    }

    fn compile_simple(scope: &mut X86Store, inst_type: InstructionType, target: Value, source: Value) -> Result<Vec<X86Instruction>, X86Error> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate source value".to_owned()));
        let mut source = X86ValueCompiler::compile(source, &mut instructions, scope, None)?;

        // Some of the operatorlar need direct register.
        let target_register = match TARGET_ALWAYS_REGISTER.contains(&inst_type) {
            true => scope.lock_register().map(|reg| X86Location::Register(AddressingMode::Direct(reg))),
            false => None
        };

        instructions.push(X86Instruction::Comment("Generate target value".to_owned()));
        let target = X86ValueCompiler::compile(target, &mut instructions, scope, target_register)?;

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().ok_or(X86Error::NoRegisterAvailable)?;
                instructions.push(X86Instruction::Mov { source, target: X86Location::Register(X86AddressingMode::Direct(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = X86Location::Register(X86AddressingMode::Direct(new_reg));
            }
        }

        let instruction = match inst_type {
            InstructionType::Add => X86Instruction::Add { source, target, comment: None },
            InstructionType::Sub => X86Instruction::Sub { source, target, comment: None },
            InstructionType::IMul => X86Instruction::IMul { source, target, comment: None },
            InstructionType::And => X86Instruction::And { source, target, comment: None },
            _ => return Err(X86Error::UnexpectedInstruction)
        };

        instructions.push(instruction);
        scope.register_restore(registers);
        scope.set_last_assigned_location(target);

        if let Some(register) = target.get_register() {
            scope.mark_register(register);
        }

        Ok(instructions)
    }

    fn compile_div(scope: &mut X86Store, divider: Value, divided: Value, target_register: Register) -> Result<Vec<X86Instruction>, X86Error> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate divider value".to_owned()));
        let mut divider = X86ValueCompiler::compile(divider, &mut instructions, scope, Some(X86Location::Register(X86AddressingMode::Direct(Register::ESI))))?;

        instructions.push(X86Instruction::Comment("Generate divided value".to_owned()));
        X86ValueCompiler::compile(divided, &mut instructions, scope, Some(X86Location::Register(X86AddressingMode::Direct(Register::EAX))))?;

        if let Some(mode) = divider.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().ok_or(X86Error::NoRegisterAvailable)?;
                instructions.push(X86Instruction::Mov { source: divider, target: X86Location::Register(X86AddressingMode::Direct(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                divider = X86Location::Register(X86AddressingMode::Direct(new_reg));
            }
        }

        instructions.push(X86Instruction::Cdq);
        instructions.push(X86Instruction::IDiv { target: divider, comment: None });
        scope.register_restore(registers);
        scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Direct(target_register)));

        if let Some(register) = divider.get_register() {
            scope.mark_register(register);
        }

        Ok(instructions)
    }

    fn compile_single(scope: &mut X86Store, inst: InstructionType, source: Value) -> Result<Vec<X86Instruction>, X86Error> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate source value".to_owned()));
        let mut source = X86ValueCompiler::compile(source, &mut instructions, scope, None)?;

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().ok_or(X86Error::NoRegisterAvailable)?;
                instructions.push(X86Instruction::Mov { source, target: X86Location::Register(X86AddressingMode::Direct(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = X86Location::Register(X86AddressingMode::Direct(new_reg));
            }
        }

        let instruction = match inst {
            InstructionType::Neg => X86Instruction::Neg { source, comment: None },
            InstructionType::Not => X86Instruction::Not { source, comment: None },
            _ => return Err(X86Error::UnexpectedInstruction)
        };

        instructions.push(instruction);
        scope.register_restore(registers);
        scope.set_last_assigned_location(source);

        if let Some(register) = source.get_register() {
            scope.mark_register(register);
        }

        Ok(instructions)
    }

    pub fn compile_value(scope: &mut X86Store, value: Value) -> Result<Vec<X86Instruction>, X86Error> {
        let mut instructions = Vec::new();
        let value = X86ValueCompiler::compile(value, &mut instructions, scope, None)?;
        scope.set_last_assigned_location(value);
        Ok(instructions)
    }
}
