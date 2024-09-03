use tb_core::{syntax::{SyntaxGeneratorTrait, TBSyntaxGenerator}, types::{AssemblyGenerator, Definition}};

use crate::{backend::Application, compiler::X86DefinitionCompiler, instruction::X86Instruction, X86ApplicationContext};


#[derive(Debug, Clone, Default)]
pub struct X86AssemblyGenerator;

impl AssemblyGenerator for X86AssemblyGenerator{
    fn generate(&self, definitions: Vec<Definition>) -> String {
        let mut context = X86ApplicationContext::default();
        let mut application = Application::default();
        
        for item in definitions.into_iter() {
            application.items.push(X86DefinitionCompiler::compile(item, &mut context).unwrap());
        }

        application.generate(&mut context);
        let syntax_generator = TBSyntaxGenerator::get_generator::<X86Instruction>().unwrap();
        syntax_generator.generate(&mut context)
    }
}

/*
- Finalize automated CM migration procedure
- CyberSecurity finding meeting and make a decition about what needs to be fixed
- Complate DRBD testing - part 2
- Finish the metrics of scheduled tasks
- Make a decition about dual CCH issue
- New CN and EZ releases
*/