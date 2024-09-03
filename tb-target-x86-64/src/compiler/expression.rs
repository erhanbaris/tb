use tb_core::{addressing_mode::AddressingMode, types::{Expression, RegisterSize, Value}};

use crate::{instruction::{X86InstructionType, X86Instruction}, register::Register, X86AddressingMode, X86Location, X86Store};

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
    pub fn compile(expression: Expression, scope: &mut X86Store) -> Result<Vec<X86Instruction>, X86Error> {
        match expression {
            Expression::Add { target, source } => Self::compile_simple(scope, X86InstructionType::Add, target, source),
            Expression::Sub { target, source } => Self::compile_simple(scope, X86InstructionType::Sub, target, source),
            Expression::Mul { target, source } => Self::compile_simple(scope, X86InstructionType::IMul, target, source),
            Expression::Modulo { divider, divided } => Self::compile_div(scope, divider, divided, Register::EDX),
            Expression::Div { divider, divided } => Self::compile_div(scope, divider, divided, Register::EAX),
            Expression::ShiftLeft { target, source } => Self::compile_simple(scope, X86InstructionType::Shl, target, source),
            Expression::ShiftRight { target, source } => Self::compile_simple(scope, X86InstructionType::Shr, target, source),
            Expression::BitwiseNot { source } => Self::compile_single(scope, X86InstructionType::Not, source),
            Expression::BitwiseAnd { source, target } => Self::compile_simple(scope, X86InstructionType::And, target, source),
            Expression::BitwiseOr { source, target } => Self::compile_simple(scope, X86InstructionType::Or, target, source),
            Expression::BitwiseXor { source, target } => Self::compile_simple(scope, X86InstructionType::Xor, target, source),
            Expression::BitwiseNeg { source } => Self::compile_single(scope, X86InstructionType::Neg, source),
            Expression::Dec { source } => Self::compile_single(scope, X86InstructionType::Dec, source),
            Expression::Inc { source } => Self::compile_single(scope, X86InstructionType::Inc, source),
            Expression::Value(val) => Self::compile_value(scope, val),
        }
    }

    fn get_target_register(scope: &mut X86Store, inst_type: X86InstructionType, get_fixed_type: fn(special_info: &SpecialConfiguration) -> Option<FixedType>) -> Option<X86Location> {
        match SPECIAL_INSTRUCTION_CHECK.iter().position(|item| *item == inst_type) {
            Some(position) => {
                let special_info = &SPECIAL_INSTRUCTION[position];
                
                // Get target type information
                match get_fixed_type(special_info) {
                    Some(FixedType::AnyRegister) => scope.lock_register().map(|reg| X86Location::Register(AddressingMode::Direct(reg))),
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

    fn compile_simple(scope: &mut X86Store, inst_type: X86InstructionType, target: Value, source: Value) -> Result<Vec<X86Instruction>, X86Error> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(X86Instruction::Comment("Generate source value".to_owned()));
        
        // Some of the operatorlar need direct register.
        let source_register = Self::get_target_register(scope, inst_type, |item| item.fixed_source_type.clone());
        
        let mut source = X86ValueCompiler::compile(source, &mut instructions, scope, source_register)?;

        // Some of the operatorlar need direct register.
        let target_register = Self::get_target_register(scope, inst_type, |item| item.fixed_target_type.clone());
        
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
            X86InstructionType::Add => X86Instruction::Add { source, target, comment: None },
            X86InstructionType::Sub => X86Instruction::Sub { source, target, comment: None },
            X86InstructionType::IMul => X86Instruction::IMul { source, target, comment: None },
            X86InstructionType::And => X86Instruction::And { source, target, comment: None },
            X86InstructionType::Or => X86Instruction::Or { source, target, comment: None },
            X86InstructionType::Xor => X86Instruction::Xor { source, target, comment: None },
            X86InstructionType::Shl => X86Instruction::Shl { source, target, comment: None },
            X86InstructionType::Shr => X86Instruction::Shr { source, target, comment: None },
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

    fn compile_single(scope: &mut X86Store, inst: X86InstructionType, source: Value) -> Result<Vec<X86Instruction>, X86Error> {
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
            X86InstructionType::Neg => X86Instruction::Neg { source, comment: None },
            X86InstructionType::Not => X86Instruction::Not { source, comment: None },
            X86InstructionType::Inc => X86Instruction::Inc { source, comment: None },
            X86InstructionType::Dec => X86Instruction::Dec { source, comment: None },
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
