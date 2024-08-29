use std::{borrow::Borrow, fmt::Debug};

use tb_core::syntax::AsmStructure;

use crate::{instruction::X86Instruction, register::{get_register_type, Register}, X86AddressingMode, X86ApplicationContext, X86Location};

pub trait AsmGenerate {
    fn generate(&self, context: &mut X86ApplicationContext, buffer: &mut String);
}


#[derive(Debug, Clone)]
pub enum Backend {
    Function { name: String, instructions: Vec<X86Instruction> },
    Instruction(X86Instruction)
}

impl AsmGenerate for Backend {
    fn generate(&self, context: &mut X86ApplicationContext, buffer: &mut String) {
        match self {
            Backend::Function { name, instructions } => self.generate_function(context, buffer, name, instructions),
            Backend::Instruction(inst) => self.generate_instruction(context, buffer, inst),
        };
    }
}

impl Backend {
    fn generate_function(&self, context: &mut X86ApplicationContext, buffer: &mut String, name: &str, instructions: &[X86Instruction]) {
        context.abstract_asms.push(AsmStructure::Branch(name.to_owned()));

        buffer.push_str(name);
        buffer.push(':');
        buffer.push_str("\r\n");

        // Function begin
        self.print_inst(X86Instruction::Push(X86Location::Register(X86AddressingMode::Immediate(Register::RBP))), context, buffer);
        self.print_inst(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Immediate(Register::RSP)), target: X86Location::Register(X86AddressingMode::Immediate(Register::RBP)), comment: None }, context, buffer);

        buffer.push_str("    # function body begin\r\n");
        context.abstract_asms.push(AsmStructure::Comment("function body begin".to_owned()));

        for instruction in instructions.iter() {
            self.print_inst(instruction, context, buffer);
        }
        buffer.push_str("    # function body end\r\n");
        context.abstract_asms.push(AsmStructure::Comment("function body end".to_owned()));

        // Function end
        self.print_inst(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Immediate(Register::RBP)), target: X86Location::Register(X86AddressingMode::Immediate(Register::RSP)), comment: None }, context, buffer);
        self.print_inst(X86Instruction::Pop(X86Location::Register(X86AddressingMode::Immediate(Register::RBP))), context, buffer);
        self.print_inst(X86Instruction::Ret, context, buffer);
    }

    fn generate_instruction(&self, context: &mut X86ApplicationContext, buffer: &mut String, inst: &X86Instruction) {
        let abstract_inst = inst.clone().convert();

        context.abstract_asms.push(AsmStructure::Instruction(abstract_inst));

        match inst {
            X86Instruction::Add { source, target, comment } => self.do_add(source, target, comment, buffer),
            X86Instruction::Not { source, comment } => self.do_not(source, comment, buffer),
            X86Instruction::Neg { source, comment } => self.do_neg(source, comment, buffer),
            X86Instruction::Mov { source, target, comment } => self.do_mov(source, target, comment, buffer),
            X86Instruction::Ret => self.do_ret(context, buffer),
            X86Instruction::Push(register) => self.do_push(register, buffer),
            X86Instruction::Pop(register) => self.do_pop(register, buffer),
            X86Instruction::Comment(comment) => self.do_comment(comment, context, buffer)
        };
        buffer.push_str("\r\n");
    }

    fn do_ret(&self, context: &mut X86ApplicationContext, buffer: &mut String) {
        context.abstract_asms.push(AsmStructure::BranchFinished);
        buffer.push_str("ret");
    }
    
    fn do_push(&self, register: &X86Location, buffer: &mut String) {
        buffer.push_str(&format!("push{} {}", self.get_suffix(&register.get_addressing_mode().unwrap()), register.get_addressing_mode().unwrap().to_string().to_lowercase()));
    }
    
    fn do_pop(&self, register: &X86Location, buffer: &mut String) {
        buffer.push_str(&format!("pop{} {}", self.get_suffix(&register.get_addressing_mode().unwrap()), register.get_addressing_mode().unwrap().to_string().to_lowercase()));
    }
    
    fn do_comment(&self, comment: &str, context: &mut X86ApplicationContext, buffer: &mut String) {
        context.abstract_asms.push(AsmStructure::Comment(comment.to_owned()));
        buffer.push_str(&self.get_comment(&Some(comment.to_owned())));
    }
    
    fn do_add(&self, source: &X86Location, target: &X86Location, comment: &Option<String>, buffer: &mut String) {
        match (source, target) {
            (X86Location::Imm(imm), X86Location::Register(register)) => buffer.push_str(&format!("add{} ${}, {} {}", self.get_suffix(register), imm, register.to_string().to_lowercase(), self.get_comment(comment))),
            (X86Location::Register(source_reg), X86Location::Register(target_reg)) => buffer.push_str(&format!("add{} {}, {} {}", self.get_suffix_from_registers(source_reg, target_reg), source_reg.to_string().to_lowercase(), target_reg.to_string().to_lowercase(), self.get_comment(comment))),
            value => panic!("unsupported ({:?})", value)
        }
    }
    
    fn do_not(&self, source: &X86Location, comment: &Option<String>, buffer: &mut String) {
        match source {
            X86Location::Register(source_reg) => buffer.push_str(&format!("not {} {}", source_reg.to_string().to_lowercase(), self.get_comment(comment))),
            _ => panic!("unsupported")
        }
    }
    
    fn do_neg(&self, source: &X86Location, comment: &Option<String>, buffer: &mut String) {
        match source {
            X86Location::Register(source_reg) => buffer.push_str(&format!("neg {} {}", source_reg.to_string().to_lowercase(), self.get_comment(comment))),
            _ => panic!("unsupported")
        }
    }
    
    fn do_mov(&self, source: &X86Location, target: &X86Location, comment: &Option<String>, buffer: &mut String) {
        match (source, target) {
            (X86Location::Imm(imm), X86Location::Register(register)) => buffer.push_str(&format!("mov{} ${}, {} {}", self.get_suffix(register), imm, register.to_string().to_lowercase(), self.get_comment(comment))),
            (X86Location::Register(source_reg), X86Location::Register(target_reg)) => buffer.push_str(&format!("mov{} {}, {} {}", self.get_suffix_from_registers(source_reg, target_reg), source_reg.to_string().to_lowercase(), target_reg.to_string().to_lowercase(), self.get_comment(comment))),
            _ => panic!("unsupported")
        }
    }

    fn print_inst<T: Borrow<X86Instruction>>(&self, inst: T, context: &mut X86ApplicationContext, buffer: &mut String) {
        buffer.push_str("    ");
        self.generate_instruction(context, buffer, inst.borrow());
    }

    fn get_comment(&self, comment: &Option<String>) -> String {
        match comment {
            Some(comment) => format!("# {}", comment),
            None => String::new()
        }
    }

    fn get_suffix(&self, mode: &X86AddressingMode) -> &str {
        match mode {
            X86AddressingMode::Immediate(_) => "",
            X86AddressingMode::Indirect(_) => "q",
            X86AddressingMode::Based(_, _) => "q",
        }
    }

    fn get_suffix_from_registers(&self, mode1: &X86AddressingMode, mode2: &X86AddressingMode) -> &str {
        let mode1_register = mode1.get_register();
        let mode2_register = mode2.get_register();

        let mode1_register_type = get_register_type(mode1_register);
        let mode2_register_type = get_register_type(mode2_register);

        match mode1_register_type != mode2_register_type {
            true => "l",
            false => ""
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Application {
    pub items: Vec<Backend>,
}

impl AsmGenerate for Application {
    fn generate(&self, context: &mut X86ApplicationContext, buffer: &mut String) {
        buffer.push_str(&format!(".globl {}\r\n", context.os_specific_defs.main_function_name()));
        for func in self.items.iter() {
            func.generate(context, buffer);
        }

        buffer.push_str(context.os_specific_defs.end_of_file_instructions());
    }
}
