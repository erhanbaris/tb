#![allow(warnings)]

use ast::{AstApplication, DefinitionType, ExpressionType, StatementType, VariableType};
use backend::{Application, ApplicationContext, AsmGenerate, Number, Instruction, Location};
use parser::{Context, Parser};

use log::{error, info, LevelFilter};
use simplelog::*;

mod ast;
// mod ast_builder;
mod backend;
mod register;
mod parser;

fn main() {
    let _ = CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]);
    let mut context = ApplicationContext::default();
    let mut ast_application = AstApplication::default();

    let assign1 = Box::new(StatementType::Assign {
        name: "test1".to_owned(),
        assigne: Box::new(ExpressionType::Add {
            source: Box::new(VariableType::Number(122)),
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

    let assign3 = Box::new(StatementType::Assign {
        name: "test3".to_owned(),
        assigne: Box::new(ExpressionType::Add {
            source: Box::new(VariableType::Variable("test1".to_owned())),
            target: Box::new(VariableType::Number(1))
        })
    });

    let assign = Box::new(StatementType::Assign {
        name: "actual".to_owned(),
        assigne: Box::new(ExpressionType::Add {
            source: Box::new(VariableType::Variable("test1".to_owned())),
            target: Box::new(VariableType::Variable("test3".to_owned()))
        })
    });

    ast_application.asts.push(Box::new(DefinitionType::Function { name: "_main".to_owned(), parameters: Vec::new(), body: vec![assign1, assign2, assign3, assign, Box::new(StatementType::Return(Some(VariableType::Variable("actual".to_owned()))))] }));
    let backend_application = ast_application.generate();
    let mut buffer = String::new();
    backend_application.generate(&mut context, &mut buffer);
    println!("{}", buffer);


    let data = br#"func main()
begin
    var $sum = 1024 add 2048
    var $mul = 1024 mul 2048
    return $sum
end"#;

    let mut context = Context::default();
    let mut parser = Parser::new(data, context);
    parser.parse().unwrap();
    //parser.friendly_dump();
}
