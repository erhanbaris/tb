use tb_core::{addressing_mode::AddressingMode, types::{Expression, RegisterSize, Value}};

use crate::{instruction::{X86Instruction, X86InstructionType}, register::Register, X86AddressingMode, X86ApplicationContext, X86Location, X86Store};

use super::{error::X86Error, value::X86ValueCompiler};

const SPECIAL_INSTRUCTION_CHECK: [X86InstructionType; 3] = [
    X86InstructionType::IMul,
    X86InstructionType::Shl,
    X86InstructionType::Shr
];
const SPECIAL_INSTRUCTION: [SpecialConfiguration; 3] = [
    SpecialConfiguration { inst: X86InstructionType::IMul, fixed_source_type: None, fixed_target_type: Some(FixedType::AnyRegister) },
    SpecialConfiguration { inst: X86InstructionType::Shl, fixed_source_type: Some(FixedType::Register(Register::ECX, Some(RegisterSize::_8Bit))), fixed_target_type: None },
    SpecialConfiguration { inst: X86InstructionType::Shr, fixed_source_type: Some(FixedType::Register(Register::ECX, Some(RegisterSize::_8Bit))), fixed_target_type: None },
];

#[derive(Debug, Clone)]
enum FixedType {
    Register(Register, Option<RegisterSize>),
    AnyRegister
}

#[derive(Debug)]
struct SpecialConfiguration {
    #[allow(dead_code)]
    pub inst: X86InstructionType,
    pub fixed_target_type: Option<FixedType>,
    pub fixed_source_type: Option<FixedType>,
}

pub struct X86ExpressionCompiler;

impl X86ExpressionCompiler {
    pub fn compile(expression: Expression, scope: &mut X86Store, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        match expression {
            Expression::Add { target, source } => Self::compile_simple(scope, X86InstructionType::Add, target, source, context),
            Expression::Sub { target, source } => Self::compile_simple(scope, X86InstructionType::Sub, target, source, context),
            Expression::Mul { target, source } => Self::compile_simple(scope, X86InstructionType::IMul, target, source, context),
            Expression::Modulo { divider, divided } => Self::compile_div(scope, divider, divided, Register::EDX, context),
            Expression::Div { divider, divided } => Self::compile_div(scope, divider, divided, Register::EAX, context),
            Expression::ShiftLeft { target, source } => Self::compile_simple(scope, X86InstructionType::Shl, target, source, context),
            Expression::ShiftRight { target, source } => Self::compile_simple(scope, X86InstructionType::Shr, target, source, context),
            Expression::BitwiseNot { source } => Self::compile_single(scope, X86InstructionType::Not, source, context),
            Expression::BitwiseAnd { source, target } => Self::compile_simple(scope, X86InstructionType::And, target, source, context),
            Expression::BitwiseOr { source, target } => Self::compile_simple(scope, X86InstructionType::Or, target, source, context),
            Expression::BitwiseXor { source, target } => Self::compile_simple(scope, X86InstructionType::Xor, target, source, context),
            Expression::BitwiseNeg { source } => Self::compile_single(scope, X86InstructionType::Neg, source, context),
            Expression::Dec { source } => Self::compile_single(scope, X86InstructionType::Dec, source, context),
            Expression::Inc { source } => Self::compile_single(scope, X86InstructionType::Inc, source, context),
            Expression::Value(val) => Self::compile_value(scope, val, context),
        }
    }

    fn get_target_register(scope: &mut X86Store, inst_type: X86InstructionType, get_fixed_type: fn(special_info: &SpecialConfiguration) -> Option<FixedType>) -> Option<X86Location> {
        match SPECIAL_INSTRUCTION_CHECK.iter().position(|item| *item == inst_type) {
            Some(position) => {
                let special_info = &SPECIAL_INSTRUCTION[position];
                
                // Get target type information
                match get_fixed_type(special_info) {
                    Some(FixedType::AnyRegister) => scope.lock_register(scope.get_last_size()).map(|reg| X86Location::Register(AddressingMode::Direct(reg))),
                    Some(FixedType::Register(expected_register, expected_size)) => {
                        // The register not used, so, we can use it
                        if scope.is_free(expected_register) {
                            scope.mark_register(expected_register);

                            // The required register has different size
                            if let Some(expected_size) = expected_size {
                                let opcode_info = expected_register.get_info();

                                let new_opcode = match expected_size {
                                    RegisterSize::_8Bit => opcode_info._8bit_low,
                                    RegisterSize::_16Bit => opcode_info._16bit,
                                    RegisterSize::_32Bit => opcode_info._32bit,
                                    RegisterSize::_64Bit => opcode_info._64bit,
                                };

                                Some(X86Location::Register(AddressingMode::Direct(new_opcode)))
                            } else {
                                Some(X86Location::Register(AddressingMode::Direct(expected_register)))
                            }
                        } else {
                            None
                        }
                    },
                    None => None,
                }
            },
            None => None
        }
    }

    fn compile_simple(scope: &mut X86Store, inst_type: X86InstructionType, target: Value, source: Value, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let registers = scope.register_backup();

        context.instructions.add_comment("Generate source value".to_owned());
        
        // Some of the operatorlar need direct register.
        let source_register = Self::get_target_register(scope, inst_type, |item| item.fixed_source_type.clone());
        
        let mut source = X86ValueCompiler::compile(source, context, scope, source_register)?;

        // Some of the operatorlar need direct register.
        let target_register = Self::get_target_register(scope, inst_type, |item| item.fixed_target_type.clone());
        
        context.instructions.add_comment("Generate target value".to_owned());
        let target = X86ValueCompiler::compile(target, context, scope, target_register)?;

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register(scope.get_last_size()).ok_or(X86Error::NoRegisterAvailable)?;
                context.instructions.add_instruction(X86Instruction::Mov { source, target: X86Location::Register(X86AddressingMode::Direct(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = X86Location::Register(X86AddressingMode::Direct(new_reg));
            }
        }

        let instruction = match inst_type {
            X86InstructionType::Add => X86Instruction::Add { source, target: target.clone(), comment: None },
            X86InstructionType::Sub => X86Instruction::Sub { source, target: target.clone(), comment: None },
            X86InstructionType::IMul => X86Instruction::IMul { source, target: target.clone(), comment: None },
            X86InstructionType::And => X86Instruction::And { source, target: target.clone(), comment: None },
            X86InstructionType::Or => X86Instruction::Or { source, target: target.clone(), comment: None },
            X86InstructionType::Xor => X86Instruction::Xor { source, target: target.clone(), comment: None },
            X86InstructionType::Shl => X86Instruction::Shl { source, target: target.clone(), comment: None },
            X86InstructionType::Shr => X86Instruction::Shr { source, target: target.clone(), comment: None },
            _ => return Err(X86Error::UnexpectedInstruction)
        };

        context.instructions.add_instruction(instruction);
        scope.register_restore(registers);
        scope.set_last_assigned_location(target.clone());

        if let Some(register) = target.get_register() {
            scope.mark_register(register);
        }

        Ok(())
    }

    fn compile_div(scope: &mut X86Store, divider: Value, divided: Value, target_register: Register, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let registers = scope.register_backup();

        context.instructions.add_comment("Generate divider value".to_owned());
        let mut divider = X86ValueCompiler::compile(divider, context, scope, Some(X86Location::Register(X86AddressingMode::Direct(Register::ESI))))?;

        context.instructions.add_comment("Generate divided value".to_owned());
        X86ValueCompiler::compile(divided, context, scope, Some(X86Location::Register(X86AddressingMode::Direct(Register::EAX))))?;

        if let Some(mode) = divider.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register(scope.get_last_size()).ok_or(X86Error::NoRegisterAvailable)?;
                context.instructions.add_instruction(X86Instruction::Mov { source: divider, target: X86Location::Register(X86AddressingMode::Direct(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                divider = X86Location::Register(X86AddressingMode::Direct(new_reg));
            }
        }

        context.instructions.add_instruction(X86Instruction::Cdq);
        context.instructions.add_instruction(X86Instruction::IDiv { target: divider.clone(), comment: None });
        scope.register_restore(registers);
        scope.set_last_assigned_location(X86Location::Register(X86AddressingMode::Direct(target_register)));

        if let Some(register) = divider.get_register() {
            scope.mark_register(register);
        }

        Ok(())
    }

    fn compile_single(scope: &mut X86Store, inst: X86InstructionType, source: Value, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let registers = scope.register_backup();

        context.instructions.add_comment("Generate source value".to_owned());
        let mut source = X86ValueCompiler::compile(source, context, scope, None)?;

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register(scope.get_last_size()).ok_or(X86Error::NoRegisterAvailable)?;
                context.instructions.add_instruction(X86Instruction::Mov { source, target: X86Location::Register(X86AddressingMode::Direct(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = X86Location::Register(X86AddressingMode::Direct(new_reg));
            }
        }

        let instruction = match inst {
            X86InstructionType::Neg => X86Instruction::Neg { source: source.clone(), comment: None },
            X86InstructionType::Not => X86Instruction::Not { source: source.clone(), comment: None },
            X86InstructionType::Inc => X86Instruction::Inc { source: source.clone(), comment: None },
            X86InstructionType::Dec => X86Instruction::Dec { source: source.clone(), comment: None },
            _ => return Err(X86Error::UnexpectedInstruction)
        };

        context.instructions.add_instruction(instruction);
        scope.register_restore(registers);
        scope.set_last_assigned_location(source.clone());

        if let Some(register) = source.get_register() {
            scope.mark_register(register);
        }

        Ok(())
    }

    pub fn compile_value(scope: &mut X86Store, value: Value, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let value = X86ValueCompiler::compile(value, context, scope, None)?;
        scope.set_last_assigned_location(value);
        Ok(())
    }
}
