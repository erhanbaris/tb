use std::fmt::Display;

#[derive(Copy, Clone, PartialEq)]
pub enum RegisterType {
    _16Bit,
    _32Bit,
    _64Bit
}

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Register {
    AX, BX, CX, DX, DI, SI, BP, SP, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W, // Word Registers
    EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D, // Doubleword Registers
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15, // Quadword Registers

    LASTELEMENT
}

pub const REGISTER_OPCODES: [u8; 49] = [
    // Word Registers
    0, 3, 1, 2, 6, 7, 5, 4, 8, 9, 10, 11, 12, 13, 14, 15,
    
    // Doubleword Registers
    0, 3, 1, 2, 6, 7, 5, 4, 8, 9, 10, 11, 12, 13, 14, 15,
    
    // Quadword Registers
    0, 3, 1, 2, 6, 7, 5, 4, 8, 9, 10, 11, 12, 13, 14, 15,
    
    // LASTELEMENT
    255
];

pub const REGISTER_TYPES: [RegisterType; 49] = [
    // Word Registers
    RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit,
    
    // Doubleword Registers
    RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit,  RegisterType::_32Bit,
    
    // Quadword Registers
    RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit,  RegisterType::_64Bit,
    
    // LASTELEMENT
    RegisterType::_64Bit
];

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum AddressingMode {
    Immediate(Register),
    Indirect(Register),
    Based(i32, Register),
    Complex // todo: later
}

impl Display for AddressingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressingMode::Immediate(reg) => write!(f, "%{:?}", reg),
            AddressingMode::Indirect(reg) => write!(f, "(%{:?})", reg),
            AddressingMode::Based(num, reg) => write!(f, "{}(%{:?})", num, reg),
            AddressingMode::Complex => todo!(),
        }
    }
}

impl AddressingMode {
    pub fn create_based(base: i32, register: Register) -> Self {
        match base {
            0 => AddressingMode::Immediate(register),
            _ => AddressingMode::Based(base, register)
        }
    }

    pub fn get_register(&self) -> Register {
        match self {
            AddressingMode::Immediate(register) => *register,
            AddressingMode::Indirect(register) => *register,
            AddressingMode::Based(_, register) => *register,
            AddressingMode::Complex => Register::RAX,
        }
    }

    pub fn is_direct_register(&self) -> bool {
        match self {
            AddressingMode::Immediate(_) => true,
            _ => false
        }
    }
}

pub fn get_register_type(register: Register) -> RegisterType {
    REGISTER_TYPES[register as usize]
}

pub fn get_register_opcode(register: Register) -> u8 {
    REGISTER_OPCODES[register as usize]
}

// Compile time checks
const _: () = assert!(REGISTER_TYPES.len() == Register::LASTELEMENT as usize + 1, "Missing register types");
const _: () = assert!(REGISTER_OPCODES.len() == Register::LASTELEMENT as usize + 1, "Missing register opcode");
