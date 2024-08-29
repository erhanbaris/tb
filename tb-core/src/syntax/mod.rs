use std::fmt::Debug;

use att_syntax::ATTSyntaxGenerator;

use crate::{error::TBError, instruction::InstructionTrait, types::ApplicationContext};

mod att_syntax;

pub trait SyntaxGeneratorTrait<I: InstructionTrait> where Self: Sized + Default {
    fn generate(&self, context: &mut ApplicationContext<I>) -> String;
}

pub struct TBSyntaxGenerator;

impl TBSyntaxGenerator {
    pub fn get_generator<I: InstructionTrait>() -> Result<Box<impl SyntaxGeneratorTrait<I>>, TBError> {
        let compiler = ATTSyntaxGenerator::default();
        Ok(Box::new(compiler))
    }
}

#[derive(Debug, Clone)]
pub enum AsmStructure<I: InstructionTrait> {
    Branch(String),
    BranchFinished,
    Comment(String),
    Instruction(Box<I>)
}
