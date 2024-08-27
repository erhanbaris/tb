mod value;
mod expression;
mod statement;
mod definition;


use definition::X86DefinitionGenerator;
use tb_core::types::{ApplicationContext, AssemblyGenerator, Definition};

use crate::backend::{Application, AsmGenerate, Instruction};


#[derive(Debug, Clone, Default)]
pub struct X86AssemblyGenerator;

impl AssemblyGenerator for X86AssemblyGenerator {
    fn generate(&self, definitions: Vec<Definition>, mut context: ApplicationContext) -> String {
        let mut application: Application = Application::default();
        for item in definitions.into_iter() {
            application.items.push(X86DefinitionGenerator::generate(item));
        }

        let mut buffer = String::new();
        application.generate(&mut context, &mut buffer);
        buffer
    }
}

pub trait BackendGenerate {
    fn generate(&self, items: &mut Vec<Instruction>);
}
