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
    
    pub fn not(source: Value) -> Self {
        Self {
            expression_type: ExpressionDiscriminant::Not,
            source: Some(Box::new(source)),
            target: None
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
            ExpressionDiscriminant::Not => Expression::Not {
                source: *self.source.unwrap()
            },
            ExpressionDiscriminant::Neg => Expression::Neg {
                source: *self.source.unwrap()
            },
            ExpressionDiscriminant::Value => Expression::Value(*self.source.unwrap())
        }
    }
}
