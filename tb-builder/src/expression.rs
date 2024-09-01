use tb_core::types::{Expression, ExpressionDiscriminant, Value};

use super::BuilderGenerate;

#[derive(Debug, Clone)]
pub struct ExpressionType {
    expression_type: ExpressionDiscriminant,
    target: Option<Box<Value>>,
    source: Option<Box<Value>>
}

impl ExpressionType {
    pub fn add(source: Value, target: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::Add,
            source: Some(Box::new(source)),
            target: Some(Box::new(target))
        }
    }

    pub fn sub(source: Value, target: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::Sub,
            source: Some(Box::new(source)),
            target: Some(Box::new(target))
        }
    }

    pub fn div(divided: Value, divider: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::Div,
            source: Some(Box::new(divided)),
            target: Some(Box::new(divider))
        }
    }

    pub fn mul(source: Value, target: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::Mul,
            source: Some(Box::new(source)),
            target: Some(Box::new(target))
        }
    }

    pub fn modulo(divided: Value, divider: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::Modulo,
            source: Some(Box::new(divided)),
            target: Some(Box::new(divider))
        }
    }
    
    pub fn bitwise_not(source: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::BitwiseNot,
            source: Some(Box::new(source)),
            target: None
        }
    }
    
    pub fn bitwise_and(source: Value, target: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::BitwiseAnd,
            source: Some(Box::new(source)),
            target: Some(Box::new(target))
        }
    }
    
    pub fn neg(source: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::Neg,
            source: Some(Box::new(source)),
            target: None
        }
    }
    
    pub fn value(source: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::Value,
            source: Some(Box::new(source)),
            target: None
        }
    }
}

impl Default for ExpressionType {
    fn default() -> Self {
        Self { expression_type: ExpressionDiscriminant::Add, target: None, source: None }
    }
}

impl BuilderGenerate for ExpressionType {
    type Output = Expression;

    fn convert(self) -> Self::Output {
        match self.expression_type {
            ExpressionDiscriminant::Add => Expression::Add {
                source: *self.source.unwrap(),
                target: *self.target.unwrap()
            },
            ExpressionDiscriminant::Sub => Expression::Sub {
                source: *self.source.unwrap(),
                target: *self.target.unwrap()
            },
            ExpressionDiscriminant::Div => Expression::Div {
                divider: *self.source.unwrap(),
                divided: *self.target.unwrap()
            },
            ExpressionDiscriminant::Mul => Expression::Mul {
                source: *self.source.unwrap(),
                target: *self.target.unwrap()
            },
            ExpressionDiscriminant::Modulo => Expression::Modulo {
                divider: *self.source.unwrap(),
                divided: *self.target.unwrap()
            },
            ExpressionDiscriminant::BitwiseAnd => Expression::BitwiseAnd {
                source: *self.source.unwrap(),
                target: *self.target.unwrap()
            },
            ExpressionDiscriminant::BitwiseNot => Expression::BitwiseNot {
                source: *self.source.unwrap()
            },
            ExpressionDiscriminant::Neg => Expression::Neg {
                source: *self.source.unwrap()
            },
            ExpressionDiscriminant::Value => Expression::Value(*self.source.unwrap())
        }
    }
}
