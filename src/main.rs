#![allow(warnings)]

use ast::{AstApplication, DefinitionType, ExpressionType, StatementType, VariableType};
use backend::{Application, ApplicationContext, AsmGenerate, Register, Number, Instruction, Location};

mod ast;
mod backend;

fn main() {
    let mut context = ApplicationContext::new();
    let mut ast_application = AstApplication::default();
    ast_application.asts.push(Box::new(DefinitionType::Function { name: "_main".to_owned(), parameters: Vec::new(), body: vec![Box::new(StatementType::Return(Some(VariableType::Number(2))))] }));
    let backend_application = ast_application.generate();
    let mut buffer = String::new();
    backend_application.generate(&mut context, &mut buffer);
    println!("{}", buffer);
}
