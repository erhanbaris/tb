use std::string::ToString;
use std::fmt::Debug;

use crate::location::Location;

#[derive(Debug, Clone)]
pub struct AbstractInstruction<I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString> {
    pub inst: I,
    pub target: Option<Location<R>>,
    pub source1: Option<Location<R>>,
    pub source2: Option<Location<R>>,
    pub comment: Option<String>
}

impl<I, R> AbstractInstruction<I, R> where I: Debug + ToString + Clone, R: Clone + PartialEq + Debug + ToString {
    pub fn simple(inst: I) -> Self {
        Self { inst, target: None, source1: None, source2: None, comment: None }
    }

    pub fn simple_with_comment(inst: I, comment: Option<String>) -> Self {
        Self { inst, target: None, source1: None, source2: None, comment }
    }

    pub fn target(inst: I, target: Location<R>) -> Self {
        Self { inst, target: Some(target), source1: None, source2: None, comment: None }
    }

    pub fn target_with_comment(inst: I, target: Location<R>, comment: Option<String>) -> Self {
        Self { inst, target: Some(target), source1: None, source2: None, comment }
    }

    pub fn target_source(inst: I, target: Location<R>, source: Location<R>) -> Self {
        Self { inst, target: Some(target), source1: Some(source), source2: None, comment: None }
    }

    pub fn target_source_with_comment(inst: I, target: Location<R>, source: Location<R>, comment: Option<String>) -> Self {
        Self { inst, target: Some(target), source1: Some(source), source2: None, comment }
    }

    pub fn target_source2(inst: I, target: Location<R>, source1: Location<R>, source2: Location<R>) -> Self {
        Self { inst, target: Some(target), source1: Some(source1), source2: Some(source2), comment: None }
    }

    pub fn target_source2_with_comment(inst: I, target: Location<R>, source1: Location<R>, source2: Location<R>, comment: Option<String>) -> Self {
        Self { inst, target: Some(target), source1: Some(source1), source2: Some(source2), comment }
    }
}
