use std::fmt::Display;

use tb_core::types::{RegisterTrait, RegisterSize};

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Register {
    AL, BL, CL, DL, AH, BH, CH, DH, DIL, SIL, BPL, SPL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B, // Byte Registers
    AX, BX, CX, DX, DI, SI, BP, SP, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W, // Word Registers
    EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D, // Doubleword Registers
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15, // Quadword Registers
}

impl Register {    
    pub fn get_info(&self) -> &'static RegisterSizes {
        match *self {
            Register::RAX | Register::EAX | Register::AX | Register::AH | Register::AL => &OPCODE_TYPES[0],
            Register::RBX | Register::EBX | Register::BX | Register::BH | Register::BL => &OPCODE_TYPES[1],
            Register::RCX | Register::ECX | Register::CX | Register::CH | Register::CL => &OPCODE_TYPES[2],
            Register::RDX | Register::EDX | Register::DX | Register::DH | Register::DL => &OPCODE_TYPES[3],
            Register::RSI | Register::ESI | Register::SI | Register::SIL => &OPCODE_TYPES[4],
            Register::RDI | Register::EDI | Register::DI | Register::DIL => &OPCODE_TYPES[5],
            Register::RBP | Register::EBP | Register::BP | Register::BPL => &OPCODE_TYPES[6],
            Register::RSP | Register::ESP | Register::SP | Register::SPL => &OPCODE_TYPES[7],
            Register::R8 | Register::R8D | Register::R8W | Register::R8B => &OPCODE_TYPES[8],
            Register::R9 | Register::R9D | Register::R9W | Register::R9B => &OPCODE_TYPES[9],
            Register::R10 | Register::R10D | Register::R10W | Register::R10B => &OPCODE_TYPES[10],
            Register::R11 | Register::R11D | Register::R11W | Register::R11B => &OPCODE_TYPES[11],
            Register::R12 | Register::R12D | Register::R12W | Register::R12B => &OPCODE_TYPES[12],
            Register::R13 | Register::R13D | Register::R13W | Register::R13B => &OPCODE_TYPES[13],
            Register::R14 | Register::R14D | Register::R14W | Register::R14B => &OPCODE_TYPES[14],
            Register::R15 | Register::R15D | Register::R15W | Register::R15B => &OPCODE_TYPES[15],

        }
    }
}

pub struct RegisterSizes {
    pub _64bit: Register,
    pub _32bit: Register,
    pub _16bit: Register,
    pub _8bit_low: Register,
    pub _8bit_high: Option<Register>,
}

pub const OPCODE_TYPES: [RegisterSizes; 16] = [
    RegisterSizes { _64bit: Register::RAX, _32bit: Register::EAX, _16bit: Register::AX, _8bit_high: Some(Register::AH), _8bit_low: Register::AL },
    RegisterSizes { _64bit: Register::RBX, _32bit: Register::EBX, _16bit: Register::BX, _8bit_high: Some(Register::BH), _8bit_low: Register::BL },
    RegisterSizes { _64bit: Register::RCX, _32bit: Register::ECX, _16bit: Register::CX, _8bit_high: Some(Register::CH), _8bit_low: Register::CL },
    RegisterSizes { _64bit: Register::RDX, _32bit: Register::EDX, _16bit: Register::DX, _8bit_high: Some(Register::DH), _8bit_low: Register::DL },
    RegisterSizes { _64bit: Register::RSI, _32bit: Register::ESI, _16bit: Register::SI, _8bit_high: None, _8bit_low: Register::SIL },
    RegisterSizes { _64bit: Register::RDI, _32bit: Register::EDI, _16bit: Register::DI, _8bit_high: None, _8bit_low: Register::DIL },
    RegisterSizes { _64bit: Register::RBP, _32bit: Register::EBP, _16bit: Register::BP, _8bit_high: None, _8bit_low: Register::BPL },
    RegisterSizes { _64bit: Register::RSP, _32bit: Register::ESP, _16bit: Register::SP, _8bit_high: None, _8bit_low: Register::SPL },
    RegisterSizes { _64bit: Register::R8,  _32bit: Register::R8D, _16bit: Register::R8W, _8bit_high: None, _8bit_low: Register::R8B },
    RegisterSizes { _64bit: Register::R9,  _32bit: Register::R9D, _16bit: Register::R9W, _8bit_high: None, _8bit_low: Register::R9B },
    RegisterSizes { _64bit: Register::R10, _32bit: Register::R10D, _16bit: Register::R10W, _8bit_high: None, _8bit_low: Register::R10B },
    RegisterSizes { _64bit: Register::R11, _32bit: Register::R11D, _16bit: Register::R11W, _8bit_high: None, _8bit_low: Register::R11B },
    RegisterSizes { _64bit: Register::R12, _32bit: Register::R12D, _16bit: Register::R12W, _8bit_high: None, _8bit_low: Register::R12B },
    RegisterSizes { _64bit: Register::R13, _32bit: Register::R13D, _16bit: Register::R13W, _8bit_high: None, _8bit_low: Register::R13B },
    RegisterSizes { _64bit: Register::R14, _32bit: Register::R14D, _16bit: Register::R14W, _8bit_high: None, _8bit_low: Register::R14B },
    RegisterSizes { _64bit: Register::R15, _32bit: Register::R15D, _16bit: Register::R15W, _8bit_high: None, _8bit_low: Register::R15B },
];

impl RegisterTrait for Register {
    fn get_register_size(&self) -> RegisterSize {
        REGISTER_SIZES[*self as usize]
    }
    
    fn get_sized(self, size: RegisterSize) -> Self {
        let info = self.get_info();
        match size {
            RegisterSize::_8Bit => info._8bit_low,
            RegisterSize::_16Bit => info._16bit,
            RegisterSize::_32Bit => info._32bit,
            RegisterSize::_64Bit => info._64bit,
        }
    }
}

pub const REGISTER_SIZES: [RegisterSize; 68] = [
    // Byte Registers
    RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit, RegisterSize::_8Bit,

    // Word Registers
    RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit, RegisterSize::_16Bit,
    
    // Doubleword Registers
    RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit, RegisterSize::_32Bit,  RegisterSize::_32Bit,
    
    // Quadword Registers
    RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit, RegisterSize::_64Bit,  RegisterSize::_64Bit,
];

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_register_type(register: Register) -> RegisterSize {
    REGISTER_SIZES[register as usize]
}

// Compile time checks
const _: () = assert!(REGISTER_SIZES.len() == Register::R15 as usize + 1, "Missing register types");
