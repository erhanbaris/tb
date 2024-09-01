use tb_core::types::{Block, Statement, Value};

use super::{expression::ExpressionType, BuilderGenerate};

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BlockType {
    items: Vec<Statement>
}

impl BlockType {
    pub fn add_assign(&mut self, name: &str, expression: ExpressionType) {
        self.items.push(Statement::Assign {
            name: name.to_owned(),
            assigne: expression.convert()
        })
    }

    pub fn add_return(&mut self) {
        self.items.push(Statement::Return(None))
    }

    pub fn add_return_number(&mut self, value: i64) {
        self.items.push(Statement::Return(Some(Value::Number(value))))
    }

    pub fn add_return_variable(&mut self, name: &str) {
        self.items.push(Statement::Return(Some(Value::Variable(name.to_owned()))))
    }
}

impl BuilderGenerate for BlockType {
    type Output = Block;

    fn convert(self) -> Self::Output {
        let Self { items } = self;
        Self::Output { items }
    }
}
