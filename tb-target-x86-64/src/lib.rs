use instruction::InstructionType;
use register::Register;
use tb_core::{addressing_mode::AddressingMode, instruction::AbstractInstruction, location::Location, store::{Store, StoreDefaultRegisters}, types::ApplicationContext};

pub mod register;
pub mod types;
pub mod backend;
pub mod instruction;

#[derive(Debug, Clone, Default)]
pub struct X86StoreDefaultRegisters;

impl StoreDefaultRegisters<Register> for X86StoreDefaultRegisters {
    fn initialize() -> Vec<(Register, bool)> {
        vec![(Register::RDX, true), (Register::RCX, true), (Register::R8, true), (Register::R9, true), (Register::RDI, true)]
    }
}

pub type X86Store = Store<Register, Location<Register>, X86StoreDefaultRegisters>;
pub type X86AddressingMode = AddressingMode<Register>;
pub type X86Location = Location<Register>;
pub type X86ApplicationContext = ApplicationContext<InstructionType, Register>;
pub type X86AbstractInstruction = AbstractInstruction<InstructionType, Register>;
