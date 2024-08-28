mod value;
mod expression;
mod statement;
mod definition;


use std::fmt::Debug;

use definition::X86DefinitionGenerator;
use tb_core::{syntax::{SyntaxGeneratorTrait, TBSyntaxGenerator}, types::{AssemblyGenerator, Definition}};

use crate::{backend::{Application, AsmGenerate}, instruction::{InstructionType, X86Instruction}, register::Register, X86ApplicationContext};


#[derive(Debug, Clone, Default)]
pub struct X86AssemblyGenerator;

impl AssemblyGenerator for X86AssemblyGenerator{
    fn generate(&self, definitions: Vec<Definition>) -> String {
        let mut context = X86ApplicationContext::default();
        let mut application = Application::default();
        for item in definitions.into_iter() {
            application.items.push(X86DefinitionGenerator::generate(item));
        }

        let mut buffer = String::new();
        application.generate(&mut context, &mut buffer);
        let syntax_generator = TBSyntaxGenerator::get_generator::<InstructionType, Register>().unwrap();
        let att_syntax = syntax_generator.generate(&mut context);
        println!("{}", att_syntax);

        buffer
    }
}

pub trait BackendGenerate {
    fn generate(&self, items: &mut Vec<X86Instruction>);
}
