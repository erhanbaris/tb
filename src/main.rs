#![allow(warnings)]

use ast::{AstApplication, DefinitionType, ExpressionType, StatementType, VariableType};
use backend::{Application, ApplicationContext, AsmGenerate, Number, Instruction, Location};

mod ast;
mod backend;
mod register;

fn main() {
    let mut context = ApplicationContext::default();
    let mut ast_application = AstApplication::default();

    let assign1 = Box::new(StatementType::Assign {
        name: "test1".to_owned(),
        assigne: Box::new(ExpressionType::Add {
            source: Box::new(VariableType::Number(10)),
            target: Box::new(VariableType::Number(1))
        })
    });

    let assign2 = Box::new(StatementType::Assign {
        name: "test2".to_owned(),
        assigne: Box::new(ExpressionType::Add {
            source: Box::new(VariableType::Number(2)),
            target: Box::new(VariableType::Number(5))
        })
    });

    let assign = Box::new(StatementType::Assign {
        name: "actual".to_owned(),
        assigne: Box::new(ExpressionType::Add {
            source: Box::new(VariableType::Variable("test1".to_owned())),
            target: Box::new(VariableType::Variable("test2".to_owned()))
        })
    });

    ast_application.asts.push(Box::new(DefinitionType::Function { name: "main".to_owned(), parameters: Vec::new(), body: vec![assign1, assign2, assign, Box::new(StatementType::Return(Some(VariableType::Variable("actual".to_owned()))))] }));
    let backend_application = ast_application.generate();
    let mut buffer = String::new();
    backend_application.generate(&mut context, &mut buffer);
    println!("{}", buffer);
}
