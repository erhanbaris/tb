use std::{cell::Cell, fmt::Debug, marker::PhantomData};

use crate::{addressing_mode::AddressingMode, instruction::{AbstractInstruction, InstructionTrait}, location::Location, types::{ApplicationContext, RegisterTrait}};

use super::{AsmStructure, SyntaxGeneratorTrait};

#[derive(Debug, Clone)]
pub struct ATTSyntaxGenerator<I> where I: InstructionTrait {
    _marker1: PhantomData<I>,
    in_branch: Cell<bool>
}

impl<I> Default for ATTSyntaxGenerator<I> where I: InstructionTrait {
    fn default() -> Self {
        Self { _marker1: PhantomData, in_branch: Default::default() }
    }
}

impl<I> SyntaxGeneratorTrait<I> for ATTSyntaxGenerator<I> where I: InstructionTrait {
    fn generate(&self, context: &mut ApplicationContext<I>) -> String {
        let mut buffer = String::new();
        buffer.push_str(&format!(".globl {}\r\n", context.os_specific_defs.main_function_name()));

        for item in context.abstract_asms.clone().into_iter() {
            self.process_item(item, context, &mut buffer);
        }

        buffer.push_str(context.os_specific_defs.end_of_file_instructions());
        buffer
    }
}

impl<I> ATTSyntaxGenerator<I> where I: InstructionTrait {
    fn process_item(&self, item: AsmStructure<I>, context: &mut ApplicationContext<I>, buffer: &mut String) {
        match item {
            AsmStructure::Branch(name) => self.generate_branch(name, context, buffer),
            AsmStructure::BranchFinished => self.in_branch.set(false),
            AsmStructure::Comment(comment) => self.generate_comment(comment, context, buffer),
            AsmStructure::Instruction(inst) => self.generate_instruction(inst.convert(), context, buffer),
        };
    }

    fn generate_location(&self, location: Location<I::REG>, buffer: &mut String) {
        buffer.push(' ');
        match location {
            Location::Memory(memory) => buffer.push_str(&format!("{:#01x}", memory)),
            Location::Register(mode) => match mode {
                AddressingMode::Direct(reg) => buffer.push_str(&format!("%{}", reg.to_string().to_lowercase())),
                AddressingMode::Indirect(reg) => buffer.push_str(&format!("(%{})", reg.to_string().to_lowercase())),
                AddressingMode::Based(num, reg) => buffer.push_str(&format!("{}(%{})", num, reg.to_string().to_lowercase())),
            },
            Location::Imm(imm) => buffer.push_str(&format!("${}", imm)),
        }
    }

    fn generate_instruction(&self, inst: AbstractInstruction<I>, _: &mut ApplicationContext<I>, buffer: &mut String) {
        if inst.inst.to_string() == "Comment" { return }
        let mut has_source = false;

        if self.in_branch.get() {
            buffer.push_str("    ");
        }

        buffer.push_str(&inst.inst.to_string().to_lowercase());

        match (&inst.target.as_ref().and_then(|item| item.get_addressing_mode()), &inst.source1.as_ref().and_then(|item| item.get_addressing_mode())) {
            (Some(target), None) => buffer.push_str(self.get_suffix(target)),
            (None, Some(source)) => buffer.push_str(self.get_suffix(source)),
            (Some(target), Some(source)) => buffer.push_str(self.get_suffix_from_registers(target, source)),
            (None, None) => ()
        };
        
        if let Some(source) = inst.source1 {
            self.generate_location(source, buffer);
            has_source = true;
        }

        if let Some(target) = inst.target {
            if has_source {
                buffer.push(',');
            }

            self.generate_location(target, buffer);
        }

        if let Some(comment) = inst.comment {
            buffer.push_str(" # ");
            buffer.push_str(&comment);
        }

        buffer.push_str("\r\n");
    }

    fn generate_branch(&self, name: String, _: &mut ApplicationContext<I>, buffer: &mut String) {
        self.in_branch.set(true);
        buffer.push_str(&name);
        buffer.push(':');
        buffer.push_str("\r\n");
    }

    fn generate_comment(&self, name: String, _: &mut ApplicationContext<I>, buffer: &mut String) {
        if self.in_branch.get() {
            buffer.push_str("    ");
        }

        buffer.push_str("# ");
        buffer.push_str(&name);
        buffer.push_str("\r\n");
    }

    fn get_suffix(&self, mode: &AddressingMode<I::REG>) -> &str {
        match mode {
            AddressingMode::Direct(_) => "",
            AddressingMode::Indirect(_) => "q",
            AddressingMode::Based(_, _) => "q",
        }
    }

    fn get_suffix_from_registers(&self, mode1: &AddressingMode<I::REG>, mode2: &AddressingMode<I::REG>) -> &str {
        let mode1_register = mode1.get_register();
        let mode2_register = mode2.get_register();

        let mode1_register_type = mode1_register.get_register_type();
        let mode2_register_type = mode2_register.get_register_type();

        match mode1_register_type != mode2_register_type {
            true => "l",
            false => ""
        }
    }
}