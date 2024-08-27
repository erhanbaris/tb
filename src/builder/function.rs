use crate::{ast::{Definition, Statement, Variable}, tool::os_defs};

use super::{expression::ExpressionType, BuilderGenerate};

#[derive(Debug, Clone, Default)]
pub struct FunctionType {
    name: String,
    parameters: Vec<Box<Variable>>,
    body: Vec<Box<Statement>>
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
        self.parameters.push(Box::new(Variable::Variable(name.to_owned())));
    }

    pub fn add_number_parameter(&mut self, value: i32) {
        self.parameters.push(Box::new(Variable::Number(value)));
    }

    pub fn add_assign(&mut self, name: &str, expression: ExpressionType) {
        self.body.push(Box::new(Statement::Assign {
            name: name.to_owned(),
            assigne: Box::new(expression.convert())
        }))
    }

    pub fn add_return(&mut self) {
        self.body.push(Box::new(Statement::Return(None)))
    }

    pub fn add_return_number(&mut self, value: i32) {
        self.body.push(Box::new(Statement::Return(Some(Variable::Number(value)))))
    }

    pub fn add_return_variable(&mut self, name: &str) {
        self.body.push(Box::new(Statement::Return(Some(Variable::Variable(name.to_owned())))))
    }
}

impl BuilderGenerate for FunctionType {
    type Output = Definition;

    fn convert(self) -> Self::Output {
        let Self { name, parameters, body } = self;
        Definition::Function { name, parameters, body }
    }
}
