use tb_core::types::{Block, Definition, Value};

use crate::{backend::Backend, instruction::X86Instruction, X86ApplicationContext, X86Store};

use super::statement::X86StatementCompiler;


pub struct X86DefinitionCompiler;

impl X86DefinitionCompiler {
    pub fn compile(definition: Definition, context: &mut X86ApplicationContext) -> Backend {
        match definition {
            Definition::Function { name, parameters, block } => Self::compile_function(name, parameters, block, context)
        }
    }

    fn compile_function(name: String, _: Vec<Value>, block: Block, context: &mut X86ApplicationContext) -> Backend {
        let mut instructions: Vec<X86Instruction> = Vec::new();
        let mut scope = X86Store::default();
        for item in block.items.into_iter() {
            instructions.append(&mut X86StatementCompiler::compile(item, &mut scope, context));
        }

        Backend::Function { name: name.clone(), instructions }
    }
}