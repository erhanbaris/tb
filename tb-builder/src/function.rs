use tb_core::{types::{Definition, Value}, tool::os_defs};

use crate::BlockType;

use super::BuilderGenerate;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FunctionType {
    name: String,
    parameters: Vec<Value>,
    block: BlockType
}

impl FunctionType {
    pub fn main() -> Self {
        let defs = os_defs();
        Self {
            name: defs.main_function_name().to_owned(),
            parameters: Default::default(),
            block: Default::default()
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn get_name(&mut self) -> &str {
        &self.name
    }

    pub fn add_variable_parameter(&mut self, name: &str) {
        self.parameters.push(Value::Variable(name.to_owned()));
    }

    pub fn add_number_parameter(&mut self, value: i32) {
        self.parameters.push(Value::Number(value));
    }

    pub fn set_body(&mut self, block: BlockType) {
        self.block = block;
    }
}

impl BuilderGenerate for FunctionType {
    type Output = Definition;

    fn convert(self) -> Self::Output {
        let Self { name, parameters, block } = self;
        Definition::Function { name, parameters, block: block.convert() }
    }
}
