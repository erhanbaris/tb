use backend::Location;
use register::Register;
use tb_core::store::{Store, StoreDefaultRegisters};

pub mod register;
pub mod types;
pub mod backend;

#[derive(Debug, Clone, Default)]
pub struct X86StoreDefaultRegisters;

impl StoreDefaultRegisters<Register> for X86StoreDefaultRegisters {
    fn initialize() -> Vec<(Register, bool)> {
        vec![(Register::RDX, true), (Register::RCX, true), (Register::R8, true), (Register::R9, true), (Register::RDI, true)]
    }
}

pub type X86Store = Store<Register, Location, X86StoreDefaultRegisters>;
