use tb_core::types::{Block, Number, Statement, Value};

use crate::if_block::IfBlockType;

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

    pub fn add_if(&mut self, if_block: IfBlockType) {
        self.items.push(Statement::If {
            condition: if_block.condition.convert(),
            true_block: if_block.true_block.convert(),
            false_block: if_block.false_block.map(|item| item.convert())
        })
    }

    pub fn add_print(&mut self, format: String, argument: Value) {
        self.items.push(Statement::Print {
            format,
            argument
        })
    }

    pub fn add_return(&mut self) {
        self.items.push(Statement::Return(None))
    }

    pub fn add_return_number(&mut self, value: Number) {
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
