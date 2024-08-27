use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use tb_builder::{ApplicationType, ExpressionType, FunctionType};
use tb_core::types::Value;
use tb_target_x86_64::types::X86AssemblyGenerator;

fn main() {
    let _ = CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]);

    let mut main_func = FunctionType::main();
    main_func.add_assign("test1", ExpressionType::add(Value::Number(122), Value::Number(1)));
    main_func.add_assign("test2", ExpressionType::add(Value::Number(2), Value::Number(5)));
    main_func.add_assign("test3", ExpressionType::add(Value::Variable("test1".to_owned()), Value::Number(1)));
    main_func.add_assign("actual", ExpressionType::add(Value::Variable("test1".to_owned()), Value::Variable("test3".to_owned())));
    main_func.add_assign("actual", ExpressionType::not(Value::Variable("actual".to_owned())));
    main_func.add_return_variable("actual");

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);
    let buffer = application_type.build::<X86AssemblyGenerator>();
    println!("{}", &buffer);
}
