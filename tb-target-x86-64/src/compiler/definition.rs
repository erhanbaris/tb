use tb_core::types::{Definition, Statement, Value};

use crate::{backend::Backend, instruction::X86Instruction, X86ApplicationContext, X86Store};

use super::statement::X86StatementCompiler;


pub struct X86DefinitionCompiler;

impl X86DefinitionCompiler {
    pub fn compile(definition: Definition, context: &mut X86ApplicationContext) -> Backend {
        match definition {
            Definition::Function { name, parameters, body } => Self::compile_function(name, parameters, body, context)
        }
    }

    fn compile_function(name: String, _: Vec<Value>, body: Vec<Statement>, context: &mut X86ApplicationContext) -> Backend {
        let mut instructions: Vec<X86Instruction> = Vec::new();
        let mut scope = X86Store::default();
        for item in body.into_iter() {
            instructions.append(&mut X86StatementCompiler::compile(item, &mut scope, context));
        }

        Backend::Function { name: name.clone(), instructions }
    }
}