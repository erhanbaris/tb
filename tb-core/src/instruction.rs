use std::string::ToString;
use std::fmt::Debug;

use crate::location::Location;
use crate::types::RegisterTrait;

#[derive(Debug, Clone)]
pub struct AbstractInstruction<I: InstructionTrait> {
    pub inst: I,
    pub target: Option<Location<I::REG>>,
    pub source1: Option<Location<I::REG>>,
    pub source2: Option<Location<I::REG>>,
    pub label: Option<String>,
    pub comment: Option<String>
}

impl<I> AbstractInstruction<I> where I: InstructionTrait {
    pub fn simple(inst: I) -> Self {
        Self { inst, target: None, source1: None, source2: None, comment: None, label: None }
    }

    pub fn label(inst: I, label: String) -> Self {
        Self { inst, target: None, source1: None, source2: None, comment: None, label: Some(label) }
    }

    pub fn simple_with_comment(inst: I, comment: Option<String>) -> Self {
        Self { inst, target: None, source1: None, source2: None, comment, label: None }
    }

    pub fn target(inst: I, target: Location<I::REG>) -> Self {
        Self { inst, target: Some(target), source1: None, source2: None, comment: None, label: None }
    }

    pub fn target_with_comment(inst: I, target: Location<I::REG>, comment: Option<String>) -> Self {
        Self { inst, target: Some(target), source1: None, source2: None, comment, label: None }
    }

    pub fn target_source(inst: I, target: Location<I::REG>, source: Location<I::REG>) -> Self {
        Self { inst, target: Some(target), source1: Some(source), source2: None, comment: None, label: None }
    }

    pub fn target_source_with_comment(inst: I, target: Location<I::REG>, source: Location<I::REG>, comment: Option<String>) -> Self {
        Self { inst, target: Some(target), source1: Some(source), source2: None, comment, label: None }
    }

    pub fn target_source2(inst: I, target: Location<I::REG>, source1: Location<I::REG>, source2: Location<I::REG>) -> Self {
        Self { inst, target: Some(target), source1: Some(source1), source2: Some(source2), comment: None, label: None }
    }

    pub fn target_source2_with_comment(inst: I, target: Location<I::REG>, source1: Location<I::REG>, source2: Location<I::REG>, comment: Option<String>) -> Self {
        Self { inst, target: Some(target), source1: Some(source1), source2: Some(source2), comment, label: None }
    }
}

pub trait InstructionTrait: Debug + ToString + Clone {
    type IT: Debug + ToString + Clone;
    type REG: RegisterTrait;
    
    fn convert(self) -> AbstractInstruction<Self>;
    fn name(&self) -> String;
}

pub trait StorageTrait: Debug + Default {

}
