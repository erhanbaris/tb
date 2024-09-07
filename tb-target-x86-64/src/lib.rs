use instruction::X86Instruction;
use register::Register;
use storage::X86Storage;
use tb_core::{addressing_mode::AddressingMode, instruction::AbstractInstruction, location::Location, store::{Store, StoreDefaultRegisters}, types::ApplicationContext};

pub mod register;
pub mod compiler;
pub mod instruction;
pub mod generator;
pub mod storage;

#[derive(Debug, Clone, Default)]
pub struct X86StoreDefaultRegisters;

impl StoreDefaultRegisters<Register> for X86StoreDefaultRegisters {
    fn initialize() -> Vec<(Register, bool)> {
        vec![(Register::RDX, true), (Register::RCX, true), (Register::R8, true), (Register::R9, true), (Register::R10, true), (Register::RDI, true)]
    }
}

pub type X86Store = Store<Register, Location<Register>, X86StoreDefaultRegisters>;
pub type X86AddressingMode = AddressingMode<Register>;
pub type X86Location = Location<Register>;
pub type X86ApplicationContext = ApplicationContext<X86Instruction, X86Storage>;
pub type X86AbstractInstruction = AbstractInstruction<X86Instruction>;
