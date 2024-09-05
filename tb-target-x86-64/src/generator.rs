use tb_core::{syntax::{SyntaxGeneratorTrait, TBSyntaxGenerator}, types::{AssemblyGenerator, DataItemCollection, Definition}};

use crate::{compiler::X86DefinitionCompiler, instruction::X86Instruction, X86ApplicationContext};


#[derive(Debug, Clone, Default)]
pub struct X86AssemblyGenerator;

impl AssemblyGenerator for X86AssemblyGenerator {
    fn generate(&self, definitions: Vec<Definition>, datas: DataItemCollection) -> String {
        let mut context = X86ApplicationContext { datas, ..Default::default() };
        
        for item in definitions.into_iter() {
            X86DefinitionCompiler::compile(item, &mut context).unwrap();
        }

        let syntax_generator = TBSyntaxGenerator::get_generator::<X86Instruction>().unwrap();
        syntax_generator.generate(&mut context)
    }
}
