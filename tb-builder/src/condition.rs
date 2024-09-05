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

    pub fn ne(left: Value, right: Value) -> Self {
        Self {
            condition_type: ConditionDiscriminant::Ne,
            right: Some(Box::new(right)),
            left: Some(Box::new(left))
        }
    }

    pub fn gr(left: Value, right: Value) -> Self {
        Self {
            condition_type: ConditionDiscriminant::Gr,
            right: Some(Box::new(right)),
            left: Some(Box::new(left))
        }
    }

    pub fn ge(left: Value, right: Value) -> Self {
        Self {
            condition_type: ConditionDiscriminant::Ge,
            right: Some(Box::new(right)),
            left: Some(Box::new(left))
        }
    }

    pub fn ls(left: Value, right: Value) -> Self {
        Self {
            condition_type: ConditionDiscriminant::Ls,
            right: Some(Box::new(right)),
            left: Some(Box::new(left))
        }
    }

    pub fn le(left: Value, right: Value) -> Self {
        Self {
            condition_type: ConditionDiscriminant::Le,
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
            },
            ConditionDiscriminant::Ne => Condition::Ne {
                left: *self.left.unwrap(),
                right: *self.right.unwrap()
            },
            ConditionDiscriminant::Gr => Condition::Gr {
                left: *self.left.unwrap(),
                right: *self.right.unwrap()
            },
            ConditionDiscriminant::Ge => Condition::Ge {
                left: *self.left.unwrap(),
                right: *self.right.unwrap()
            },
            ConditionDiscriminant::Ls => Condition::Ls {
                left: *self.left.unwrap(),
                right: *self.right.unwrap()
            },
            ConditionDiscriminant::Le => Condition::Le {
                left: *self.left.unwrap(),
                right: *self.right.unwrap()
            }
        }
    }
}
