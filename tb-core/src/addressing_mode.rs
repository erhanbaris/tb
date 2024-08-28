use std::fmt::{Debug, Display};



#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum AddressingMode<R: Clone + PartialEq + Debug> {
    Immediate(R),
    Indirect(R),
    Based(i32, R)
}

impl<R> Display for AddressingMode<R> where R: Clone + PartialEq + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressingMode::Immediate(reg) => write!(f, "%{:?}", reg),
            AddressingMode::Indirect(reg) => write!(f, "(%{:?})", reg),
            AddressingMode::Based(num, reg) => write!(f, "{}(%{:?})", num, reg)
        }
    }
}

impl<R> AddressingMode<R> where R: Clone + PartialEq + Debug {
    pub fn create_based(base: i32, register: R) -> Self {
        match base {
            0 => AddressingMode::Immediate(register),
            _ => AddressingMode::Based(base, register)
        }
    }

    pub fn get_register(&self) -> R {
        match self {
            AddressingMode::Immediate(register) => register.clone(),
            AddressingMode::Indirect(register) => register.clone(),
            AddressingMode::Based(_, register) => register.clone(),
        }
    }

    pub fn is_direct_register(&self) -> bool {
        matches!(self, AddressingMode::Immediate(_))
    }
}
