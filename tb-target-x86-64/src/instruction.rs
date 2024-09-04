use std::fmt::Debug;

use strum_macros::EnumDiscriminants;
use strum_macros::Display;
use tb_core::instruction::InstructionTrait;

use crate::register::Register;
use crate::X86AbstractInstruction;
use crate::X86Location;

#[derive(Debug, Clone, Display, EnumDiscriminants)]
#[strum_discriminants(name(X86InstructionType))]
#[strum_discriminants(derive(Display))]
pub enum X86Instruction {
    Add {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    Sub {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    IMul {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    IDiv {
        target: X86Location,
        comment: Option<String>
    },
    Not {
        source: X86Location,
        comment: Option<String>
    },
    Neg {
        source: X86Location,
        comment: Option<String>
    },
    Inc {
        source: X86Location,
        comment: Option<String>
    },
    Dec {
        source: X86Location,
        comment: Option<String>
    },
    Mov {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    And {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    Or {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    Xor {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    Shl {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    Shr {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    Cmp {
        left: X86Location,
        right: X86Location,
        comment: Option<String>
    },
    Jne(String),
    Jmp(String),
    Cdq,
    Push(X86Location),
    Pop(X86Location),
    Ret
}

impl From<X86Instruction> for X86AbstractInstruction {
    fn from(value: X86Instruction) -> Self {
        value.convert()
    }
}

impl InstructionTrait for X86Instruction {
    type IT = X86InstructionType;
    type REG = Register;

    fn convert(self) -> X86AbstractInstruction {
        match self.clone() {
            X86Instruction::Add { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Sub { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::IDiv { target, comment } => X86AbstractInstruction::target_with_comment(self, target, comment),
            X86Instruction::IMul { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Not { source, comment } => X86AbstractInstruction::target_with_comment(self, source, comment),
            X86Instruction::Neg { source, comment } => X86AbstractInstruction::target_with_comment(self, source, comment),
            X86Instruction::Inc { source, comment } => X86AbstractInstruction::target_with_comment(self, source, comment),
            X86Instruction::Dec { source, comment } => X86AbstractInstruction::target_with_comment(self, source, comment),
            X86Instruction::And { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Or { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Xor { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Shl { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Shr { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Mov { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Cmp { left, right, comment } => X86AbstractInstruction::target_source_with_comment(self, left, right, comment),
            X86Instruction::Push(target) => X86AbstractInstruction::target(self, target),
            X86Instruction::Pop(target) => X86AbstractInstruction::target(self, target),
            X86Instruction::Jne(label) => X86AbstractInstruction::label(self, label),
            X86Instruction::Jmp(label) => X86AbstractInstruction::label(self, label),
            X86Instruction::Ret => X86AbstractInstruction::simple(self),
            X86Instruction::Cdq => X86AbstractInstruction::simple(self)
        }
    }
    
    fn name(&self) -> String {
        let t: X86InstructionType = self.into();
        t.to_string()
    }
}
