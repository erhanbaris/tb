use tb_core::{syntax::{SyntaxGeneratorTrait, TBSyntaxGenerator}, types::{AssemblyGenerator, Definition}};

use crate::{backend::Application, compiler::X86DefinitionCompiler, instruction::X86Instruction, X86ApplicationContext};


#[derive(Debug, Clone, Default)]
pub struct X86AssemblyGenerator;

impl AssemblyGenerator for X86AssemblyGenerator{
    fn generate(&self, definitions: Vec<Definition>) -> String {
        let mut context = X86ApplicationContext::default();
        let mut application = Application::default();
        
        for item in definitions.into_iter() {
            application.items.push(X86DefinitionCompiler::compile(item));
        }

        application.generate(&mut context);
        let syntax_generator = TBSyntaxGenerator::get_generator::<X86Instruction>().unwrap();
        syntax_generator.generate(&mut context)
    }
}