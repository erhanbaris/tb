use tb_core::types::{Definition, Statement, Value};

use crate::{backend::Backend, instruction::X86Instruction, X86Store};

use super::statement::X86StatementGenerator;


pub struct X86DefinitionGenerator;

impl X86DefinitionGenerator {
    pub fn generate(definition: Definition) -> Backend {
        match definition {
            Definition::Function { name, parameters, body } => Self::generate_function(name, parameters, body)
        }
    }

    fn generate_function(name: String, _: Vec<Value>, body: Vec<Statement>) -> Backend {
        let mut instructions: Vec<X86Instruction> = Vec::new();
        let mut scope = X86Store::default();
        for item in body.into_iter() {
            instructions.append(&mut X86StatementGenerator::generate(item, &mut scope));
        }

        Backend::Function { name: name.clone(), instructions }
    }
}