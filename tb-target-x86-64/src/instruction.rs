use std::fmt::Debug;

use strum_macros::EnumDiscriminants;
use strum_macros::Display;
use tb_core::instruction::InstructionTrait;

use crate::register::Register;
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
    Mov {
        source: X86Location,
        target: X86Location,
        comment: Option<String>
    },
    Cdq,
    Push(X86Location),
    Pop(X86Location),
    Comment(String),
    Ret
}

impl InstructionTrait for X86Instruction {
    type IT = InstructionType;
    type REG = Register;

    fn convert(self) -> X86AbstractInstruction {
        match self.clone() {
            X86Instruction::Add { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Sub { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::IDiv { target, comment } => X86AbstractInstruction::target_with_comment(self, target, comment),
            X86Instruction::IMul { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Not { source, comment } => X86AbstractInstruction::target_with_comment(self, source, comment),
            X86Instruction::Neg { source, comment } => X86AbstractInstruction::target_with_comment(self, source, comment),
            X86Instruction::Mov { source, target, comment } => X86AbstractInstruction::target_source_with_comment(self, target, source, comment),
            X86Instruction::Push(target) => X86AbstractInstruction::target(self, target),
            X86Instruction::Pop(target) => X86AbstractInstruction::target(self, target),
            X86Instruction::Comment(comment) => X86AbstractInstruction::simple_with_comment(self, Some(comment)),
            X86Instruction::Ret => X86AbstractInstruction::simple(self),
            X86Instruction::Cdq => X86AbstractInstruction::simple(self)
        }
    }
    
    fn name(&self) -> String {
        let t: InstructionType = self.into();
        t.to_string()
    }
}