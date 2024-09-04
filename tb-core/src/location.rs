use std::fmt::Debug;

use crate::types::{Number, RegisterTrait};
use crate::addressing_mode::AddressingMode;

#[derive(Debug, Copy, Clone)]
pub enum Location<R: RegisterTrait> {
    Memory(i64),
    Register(AddressingMode<R>),
    Imm(Number)
}

impl<R> Default for Location<R> where R: RegisterTrait {
    fn default() -> Self {
        Location::Imm(Number::I32(0))
    }
}

impl<R> Location<R> where R: RegisterTrait {
    pub fn get_register(&self) -> Option<R> {
        match self {
            Location::Register(AddressingMode::Direct(register)) => Some(register.clone()),
            Location::Register(AddressingMode::Indirect(register)) => Some(register.clone()),
            Location::Register(AddressingMode::Based(_, register)) => Some(register.clone()),
            _ => None
        }
    }
    
    pub fn get_addressing_mode(&self) -> Option<AddressingMode<R>> {
        match self {
            Location::Register(addressing_mode) => Some(addressing_mode.clone()),
            _ => None
        }
    }
}