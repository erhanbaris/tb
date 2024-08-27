use tb_core::{types::{Definition, Statement, Value}, tool::os_defs};

use super::{expression::ExpressionType, BuilderGenerate};

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FunctionType {
    name: String,
    parameters: Vec<Value>,
    body: Vec<Statement>
}

impl FunctionType {
    pub fn main() -> Self {
        let defs = os_defs();
        Self {
            name: defs.main_function_name().to_owned(),
            parameters: Default::default(),
            body: Default::default()
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

    pub fn add_assign(&mut self, name: &str, expression: ExpressionType) {
        self.body.push(Statement::Assign {
            name: name.to_owned(),
            assigne: expression.convert()
        })
    }

    pub fn add_return(&mut self) {
        self.body.push(Statement::Return(None))
    }

    pub fn add_return_number(&mut self, value: i32) {
        self.body.push(Statement::Return(Some(Value::Number(value))))
    }

    pub fn add_return_variable(&mut self, name: &str) {
        self.body.push(Statement::Return(Some(Value::Variable(name.to_owned()))))
    }
}

impl BuilderGenerate for FunctionType {
    type Output = Definition;

    fn convert(self) -> Self::Output {
        let Self { name, parameters, body } = self;
        Definition::Function { name, parameters, body }
    }
}
