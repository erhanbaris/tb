use tb_core::types::{Block, Definition, Value};

use crate::{instruction::X86Instruction, register::Register, X86AddressingMode, X86ApplicationContext, X86Location, X86Store};

use super::{error::X86Error, statement::X86StatementCompiler};


pub struct X86DefinitionCompiler;

impl X86DefinitionCompiler {
    pub fn compile(definition: Definition, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        match definition {
            Definition::Function { name, parameters, block } => Self::compile_function(name, parameters, block, context),
        }
    }

    fn compile_function(name: String, _: Vec<Value>, block: Block, context: &mut X86ApplicationContext) -> Result<(), X86Error> {
        let mut scope = X86Store::default();
        context.instructions.add_branch(name.to_owned());

        // Function begin
        context.instructions.add_instruction(X86Instruction::Push(X86Location::Register(X86AddressingMode::Direct(Register::RBP))));
        context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Direct(Register::RSP)), target: X86Location::Register(X86AddressingMode::Direct(Register::RBP)), comment: None });

        context.instructions.add_comment("function body begin".to_owned());
        
        
        for item in block.items.into_iter() {
            X86StatementCompiler::compile(item, &mut scope, context)?;
        }

        context.instructions.add_comment("function body end".to_owned());

        // Function end
        context.instructions.add_instruction(X86Instruction::Mov { source: X86Location::Register(X86AddressingMode::Direct(Register::RBP)), target: X86Location::Register(X86AddressingMode::Direct(Register::RSP)), comment: None });
        context.instructions.add_instruction(X86Instruction::Pop(X86Location::Register(X86AddressingMode::Direct(Register::RBP))));
        context.instructions.add_instruction(X86Instruction::Ret);
        context.instructions.add_close_branch();

        Ok(())
    }
}
