use tb_core::types::{ConditionDiscriminant, Condition, Value};

use super::BuilderGenerate;

#[derive(Debug, Clone)]
pub struct ConditionType {
    pub condition_type: ConditionDiscriminant,
    pub left: Option<Box<Value>>,
    pub right: Option<Box<Value>>
}

impl ConditionType {
    pub fn eq(left: Value, right: Value) -> Self {
        Self {
            condition_type: ConditionDiscriminant::Eq,
            right: Some(Box::new(right)),
            left: Some(Box::new(left))
        }
    }
}

impl Default for ConditionType {
    fn default() -> Self {
        Self { condition_type: ConditionDiscriminant::Eq, left: None, right: None }
    }
}

impl BuilderGenerate for ConditionType {
    type Output = Condition;

    fn convert(self) -> Self::Output {
        match self.condition_type {
            ConditionDiscriminant::Eq => Condition::Eq {
                left: *self.left.unwrap(),
                right: *self.right.unwrap()
            }
        }
    }
}
