use std::fmt::Debug;

use strum_macros::EnumDiscriminants;
use strum_macros::Display;

use crate::X86AbstractInstruction;
use crate::X86Location;

#[derive(Debug, Clone, Display, EnumDiscriminants)]
#[strum_discriminants(name(InstructionType))]
#[strum_discriminants(derive(Display))]
pub enum X86Instruction {
    Add {
        source: X86Location,
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
    Mov {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },

    Push(X86Location),
    Pop(X86Location),
    Comment(String),
    Ret
}

impl X86Instruction {
    pub fn convert(self) -> X86AbstractInstruction {
        match self {
            X86Instruction::Add { source, target, comment } => X86AbstractInstruction::target_source_with_comment(InstructionType::Add, target, source, comment),
            X86Instruction::Not { source, comment } => X86AbstractInstruction::target_with_comment(InstructionType::Not, source, comment),
            X86Instruction::Neg { source, comment } => X86AbstractInstruction::target_with_comment(InstructionType::Neg, source, comment),
            X86Instruction::Mov { source, target, comment } => X86AbstractInstruction::target_source_with_comment(InstructionType::Mov, target, source, comment),
            X86Instruction::Push(target) => X86AbstractInstruction::target(InstructionType::Push, target),
            X86Instruction::Pop(target) => X86AbstractInstruction::target(InstructionType::Pop, target),
            X86Instruction::Comment(comment) => X86AbstractInstruction::simple_with_comment(InstructionType::Comment, Some(comment)),
            X86Instruction::Ret => X86AbstractInstruction::simple(InstructionType::Ret)
        }
    }
}
