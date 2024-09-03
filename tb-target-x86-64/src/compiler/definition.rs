use tb_core::types::{Block, Definition, Value};

use crate::{backend::Backend, instruction::X86Instruction, X86ApplicationContext, X86Store};

use super::{error::X86Error, statement::X86StatementCompiler};


pub struct X86DefinitionCompiler;

impl X86DefinitionCompiler {
    pub fn compile(definition: Definition, context: &mut X86ApplicationContext) -> Result<Backend, X86Error> {
        match definition {
            Definition::Function { name, parameters, block } => Self::compile_function(name, parameters, block, context),
        }
    }

    fn compile_function(name: String, _: Vec<Value>, block: Block, context: &mut X86ApplicationContext) -> Result<Backend, X86Error> {
        let mut instructions: Vec<X86Instruction> = Vec::new();
        let mut scope = X86Store::default();
        for item in block.items.into_iter() {
            instructions.append(&mut X86StatementCompiler::compile(item, &mut scope, context)?);
        }

        Ok(Backend::Function { name: name.clone(), instructions })
    }
}
