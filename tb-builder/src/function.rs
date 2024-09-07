use tb_core::{tool::os_defs, types::{Definition, NumberType, Parameter}};

use crate::BlockType;

use super::BuilderGenerate;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FunctionType {
    name: String,
    parameters: Vec<Parameter>,
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

    pub fn add_parameter(&mut self, name: &str, param_type: NumberType) {
        self.parameters.push(Parameter { name: name.to_owned(), param_type });
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
