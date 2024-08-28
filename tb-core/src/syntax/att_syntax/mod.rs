use std::{fmt::Debug, marker::PhantomData};

use crate::{addressing_mode::AddressingMode, instruction::AbstractInstruction, location::Location, types::ApplicationContext};

use super::{AsmStructure, SyntaxGeneratorTrait};

#[derive(Debug, Clone)]
pub struct ATTSyntaxGenerator<I, R> where I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString {
    _marker1: PhantomData<I>,
    _marker2: PhantomData<R>
}

impl<I, R> Default for ATTSyntaxGenerator<I, R> where I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString {
    fn default() -> Self {
        Self { _marker1: PhantomData, _marker2: PhantomData }
    }
}

impl<I, R> SyntaxGeneratorTrait<I, R> for ATTSyntaxGenerator<I, R> where I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString {
    fn generate(&self, context: &mut ApplicationContext<I, R>) -> String {
        let mut buffer = String::new();

        for item in context.abstract_asms.clone().into_iter() {
            self.process_item(item, context, &mut buffer);
        }
        buffer
    }
}

impl<I, R> ATTSyntaxGenerator<I, R> where I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString {
    fn process_item(&self, item: AsmStructure<I, R>, context: &mut ApplicationContext<I, R>, buffer: &mut String) {
        match item {
            AsmStructure::Branch(name) => self.generate_branch(name, context, buffer),
            AsmStructure::Comment(comment) => self.generate_comment(comment, context, buffer),
            AsmStructure::Instruction(inst) => self.generate_instruction(inst, context, buffer),
        };
    }

    fn generate_location(&self, location: Location<R>, buffer: &mut String) {
        match location {
            Location::Memory(memory) => buffer.push_str(&format!(" {:#01x}", memory)),
            Location::Register(mode) => match mode {
                AddressingMode::Immediate(reg) => buffer.push_str(&format!(" %{:?}", reg)),
                AddressingMode::Indirect(reg) => buffer.push_str(&format!(" (%{:?})", reg)),
                AddressingMode::Based(num, reg) => buffer.push_str(&format!(" {}(%{:?})", num, reg)),
            },
            Location::Imm(imm) => buffer.push_str(&format!(" ${}", imm)),
        }
    }

    fn generate_instruction(&self, inst: AbstractInstruction<I, R>, _: &mut ApplicationContext<I, R>, buffer: &mut String) {
        let mut has_source = false;
        buffer.push_str(&inst.inst.to_string().to_lowercase());
        
        if let Some(source) = inst.source1 {
            self.generate_location(source, buffer);
            has_source = true;
        }

        if let Some(source) = inst.source2 {
            if has_source {
                buffer.push_str(", ");
            }
            
            self.generate_location(source, buffer);
        }

        if let Some(target) = inst.target {
            if has_source {
                buffer.push_str(", ");
            }

            self.generate_location(target, buffer);
        }

        buffer.push_str("\r\n");
    }

    fn generate_branch(&self, name: String, _: &mut ApplicationContext<I, R>, buffer: &mut String) {
        buffer.push_str(&name);
        buffer.push(':');
        buffer.push_str("\r\n");
    }

    fn generate_comment(&self, name: String, _: &mut ApplicationContext<I, R>, buffer: &mut String) {
        buffer.push_str("# ");
        buffer.push_str(&name);
        buffer.push_str("\r\n");
    }
}