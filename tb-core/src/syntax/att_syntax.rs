use std::{cell::Cell, fmt::Debug, marker::PhantomData};

use crate::{addressing_mode::AddressingMode, instruction::{AbstractInstruction, InstructionTrait, StorageTrait}, location::Location, types::{ApplicationContext, RegisterSize, RegisterTrait}};

use super::{AsmStructure, DataItem, SyntaxGeneratorTrait};

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
    fn generate<S: StorageTrait>(&self, context: &mut ApplicationContext<I, S>) -> String {
        let mut buffer = String::new();

        for item in context.datas.items.clone().into_iter() {
            self.process_data(item, &mut buffer, context);
        }

        buffer.push_str("\r\n.text\r\n");
        buffer.push_str(&format!(".globl {}\r\n", context.os_specific_defs.main_function_name()));
        for item in context.instructions.items.clone().into_iter() {
            self.process_item(item, context, &mut buffer);
        }

        buffer.push_str(context.os_specific_defs.end_of_file_instructions());
        buffer
    }
}

impl<I> ATTSyntaxGenerator<I> where I: InstructionTrait {
    fn process_data<S: StorageTrait>(&self, item: DataItem, buffer: &mut String, context: &mut ApplicationContext<I, S>) {
        buffer.push_str(&format!("\r\n{}\r\n.{}:\r\n", context.os_specific_defs.readonly_string_section(), &item.label));
        for data in item.values.clone().into_iter() {
            match data {
                super::Data::String(data) => buffer.push_str(&format!("    .string \"{}\"\r\n", &data.replace("\"", "\\\""))),
                super::Data::Byte(data) => buffer.push_str(&format!("    .byte {}\r\n", &data)),
            }
        }
    }

    fn process_item<S: StorageTrait>(&self, item: AsmStructure<I>, context: &mut ApplicationContext<I, S>, buffer: &mut String) {
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
            Location::Label(label) => buffer.push_str(&format!(".{}(%rip)", label)),
        }
    }

    fn generate_instruction<S: StorageTrait>(&self, inst: AbstractInstruction<I>, _: &mut ApplicationContext<I, S>, buffer: &mut String) {
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

        if let Some(label) = inst.label.as_ref() {
            buffer.push_str(&format!(" {}", label)); 
        }
        
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

    fn generate_branch<S: StorageTrait>(&self, name: String, _: &mut ApplicationContext<I, S>, buffer: &mut String) {
        self.in_branch.set(true);
        buffer.push_str(&name);
        buffer.push(':');
        buffer.push_str("\r\n");
    }

    fn generate_comment<S: StorageTrait>(&self, name: String, _: &mut ApplicationContext<I, S>, buffer: &mut String) {
        if self.in_branch.get() {
            buffer.push_str("    ");
        }

        buffer.push_str("# ");
        buffer.push_str(&name);
        buffer.push_str("\r\n");
    }

    fn get_suffix(&self,  mode: &AddressingMode<I::REG>) -> &str {
        let register = mode.get_register();
        let register_size = register.get_register_size();
        match register_size {
            RegisterSize::_8Bit => "b",
            RegisterSize::_16Bit => "w",
            RegisterSize::_32Bit => "l",
            RegisterSize::_64Bit => "q",
        }
    }

    fn get_suffix_from_registers(&self, target: &AddressingMode<I::REG>, source: &AddressingMode<I::REG>) -> &str {
        let target_register = target.get_register();
        let source_register = source.get_register();

        if target.is_direct_register() && source.is_direct_register() {
            return ""
        }

        let target_register_size = target_register.get_register_size();
        let source_register_size = source_register.get_register_size();

        match target_register_size != source_register_size {
            true => match std::cmp::min(source_register_size, target_register_size) {
                RegisterSize::_8Bit => "b",
                RegisterSize::_16Bit => "w",
                RegisterSize::_32Bit => "l",
                RegisterSize::_64Bit => "q",
            },
            false => ""
        }
    }
}