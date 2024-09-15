use tb_core::types::{Statement, Value};


use super::BuilderGenerate;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CallType {
    pub name: String,
    pub arguments: Vec<Value>,
    pub assign: Option<String>,
    pub is_variadic: bool
}

impl CallType {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_argument(&mut self, arg: Value) {
        self.arguments.push(arg);
    }

    pub fn assign_to(&mut self, variable_name: String) {
        self.assign = Some(variable_name);
    }

    pub fn set_variadic(&mut self) {
        self.is_variadic = true;
    }
}

impl BuilderGenerate for CallType {
    type Output = Statement;

    fn convert(self) -> Self::Output {
        let Self { name, arguments, assign, is_variadic } = self;
        Statement::Call { name, arguments, assign, is_variadic }
    }
}
