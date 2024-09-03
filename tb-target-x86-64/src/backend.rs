use std::{borrow::Borrow, fmt::Debug};

use tb_core::syntax::AsmStructure;

use crate::{instruction::X86Instruction, register::Register, X86AddressingMode, X86ApplicationContext, X86Location};

#[derive(Debug, Clone)]
pub enum Backend {
    Function { name: String, instructions: Vec<X86Instruction> },
    Instruction(X86Instruction)
}

impl Backend {
    fn generate(&self, context: &mut X86ApplicationContext) {
        match self {
            Backend::Function { name, instructions } => self.generate_function(context, name, instructions),
            Backend::Instruction(inst) => self.generate_instruction(inst, context),
        };
    }

    fn generate_function(&self, context: &mut X86ApplicationContext, name: &String, instructions: &[X86Instruction]) {
        context.instructions.add_branch(name.to_owned());
        context.abstract_asms.push(AsmStructure::Branch(name.to_owned()));

        // Function begin
        self.generate_instruction(X86Instruction::Push(X86Location::Register(X86AddressingMode::Direct(Register::RBP))), context);
        self.generate_instruction(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Direct(Register::RSP)), target: X86Location::Register(X86AddressingMode::Direct(Register::RBP)), comment: None }, context);

        context.abstract_asms.push(AsmStructure::Comment("function body begin".to_owned()));

        for instruction in instructions.iter() {
            self.generate_instruction(instruction, context);
        }
        context.abstract_asms.push(AsmStructure::Comment("function body end".to_owned()));

        // Function end
        self.generate_instruction(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Direct(Register::RBP)), target: X86Location::Register(X86AddressingMode::Direct(Register::RSP)), comment: None }, context);
        self.generate_instruction(X86Instruction::Pop(X86Location::Register(X86AddressingMode::Direct(Register::RBP))), context);
        self.generate_instruction(X86Instruction::Ret, context);
        context.instructions.add_close_branch();
    }

    fn generate_instruction<T: Borrow<X86Instruction>>(&self, inst: T, context: &mut X86ApplicationContext) {
        context.instructions.add_instruction(inst.borrow().clone());

        let inst = match inst.borrow() {
            X86Instruction::Comment(comment) => AsmStructure::Comment(comment.to_owned()),
            inst => AsmStructure::Instruction(Box::new(inst.clone()))
        };
        context.abstract_asms.push(inst.clone());

    }

}

#[derive(Debug, Clone, Default)]
pub struct Application {
    pub items: Vec<Backend>,
}

impl Application {
    pub fn generate(&self, context: &mut X86ApplicationContext) {
        for func in self.items.iter() {
            func.generate(context);
        }
    }
}
