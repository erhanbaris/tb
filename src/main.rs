#![allow(warnings)]

use ast::{AstApplication, Definition, Expression, Statement, Variable};
use backend::{Application, ApplicationContext, AsmGenerate, Number, Instruction, Location};
use builder::{ApplicationType, ExpressionType, FunctionType};

use log::{error, info, LevelFilter};
use simplelog::*;

mod ast;
mod builder;
// mod ast_builder;
mod backend;
mod register;
mod tool;
mod ffi;

fn main() {
    let _ = CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]);
    let mut context = ApplicationContext::default();
    let mut ast_application = AstApplication::default();

    let mut main_func = FunctionType::main();
    main_func.add_assign("test1", ExpressionType::add(Variable::Number(122), Variable::Number(1)));
    main_func.add_assign("test2", ExpressionType::add(Variable::Number(2), Variable::Number(5)));
    main_func.add_assign("test3", ExpressionType::add(Variable::Variable("test1".to_owned()), Variable::Number(1)));
    main_func.add_assign("actual", ExpressionType::add(Variable::Variable("test1".to_owned()), Variable::Variable("test3".to_owned())));
    main_func.add_assign("actual", ExpressionType::not(Variable::Variable("actual".to_owned())));
    main_func.add_return_variable("actual");

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);
    let buffer = application_type.build();
    println!("{}", &buffer);
}
