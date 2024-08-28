use std::fmt::Debug;

use att_syntax::ATTSyntaxGenerator;

use crate::{error::TBError, instruction::AbstractInstruction, types::ApplicationContext};

mod att_syntax;

pub trait SyntaxGeneratorTrait<I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString> where Self: Sized + Default {
    fn generate(&self, context: &mut ApplicationContext<I, R>) -> String;
}

pub struct TBSyntaxGenerator;

impl TBSyntaxGenerator {
    pub fn get_generator<I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString>() -> Result<Box<impl SyntaxGeneratorTrait<I, R>>, TBError> {
        let compiler = ATTSyntaxGenerator::default();
        Ok(Box::new(compiler))
    }
}

#[derive(Debug, Clone)]
pub enum AsmStructure<I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString> {
    Branch(String),
    Comment(String),
    Instruction(AbstractInstruction<I, R>)
}
