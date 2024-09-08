use core::str;
use std::{fs::File, io::Write, path::PathBuf, process::Command, str::FromStr};

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use tb_builder::{ApplicationType, BlockType, ConditionType, ExpressionType, FunctionType, IfBlockType};
use tb_core::{compiler::{CompilerTrait, TBCompiler}, types::{Number, NumberType, Value}};
use tb_target_x86_64::generator::X86AssemblyGenerator;

#[cfg(test)]
mod tests;

fn main() {
    let _ = CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]);

    let mut sum_func = FunctionType::main();
    sum_func.add_parameter("a", NumberType::I64);
    sum_func.add_parameter("b", NumberType::I64);

    let mut sum_func_block = BlockType::default();
    sum_func_block.add_assign("actual", ExpressionType::add(Value::Variable("a".to_string()), Value::Variable("b".to_string())));
    sum_func_block.add_print("Sum : %d\r\n".to_owned(), Some(Value::Variable("actual".to_owned())));
    sum_func_block.add_return_variable("actual");

    sum_func.set_body(sum_func_block);
    sum_func.set_name("sum");


    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));
    if_condition_true_block.add_print("test1 should 1, : %d\r\n".to_owned(), Some(Value::Variable("test1".to_owned())));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));
    if_condition_false_block.add_print("test1 should 0, : %d\r\n".to_owned(), Some(Value::Variable("test1".to_owned())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ne(Value::Number(Number::U64(10)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    //main_func_block.add_print("Integer value: %d\r\n".to_owned(), Some(Value::Number(Number::U32(1024))));
    //main_func_block.add_print("Merhaba\r\n".to_owned(), None); // this is not working now
    main_func_block.add_call("_printf".to_owned(), vec!["1.: %s, 2.: %s, 3.: %s, 4.: %s, 5.: %s, 6.: %s, 7.: %s, 8.: %s, 9.: %s, 10.: %s\r\n".into(), "1".into(), "2".into(), "3".into(), "4".into(), "5".into(), "6".into(), "7".into(), "8".into(), "9".into(), "10".into()]); // this is not working now
    
    main_func_block.add_call_and_assign("sum".to_owned(), vec![20.into(), 12.into()], "total".to_string()); // this is not working now
    //main_func_block.add_print("Total value: %d\r\n".to_owned(), Some(Value::Variable("total".to_string())));
    //main_func_block.add_return_variable("total");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);
    application_type.add_function(sum_func);
    //application_type.add_string_data("string1", "This is a string1.");
    //application_type.add_byte_data("string1", 1);
    
    //application_type.add_string_data("string2", "This is a string2.");
    //application_type.add_byte_data("string2", 2);

    let buffer = application_type.build::<X86AssemblyGenerator>();
    // println!("{}", &buffer);

    let mut file = File::create("out.s").unwrap();
    file.write_all(buffer.as_bytes()).unwrap();

    let compiler = TBCompiler::get_compiler().unwrap();
    compiler.compile(&PathBuf::from_str("out.s").unwrap(), &PathBuf::from_str("out.exe").unwrap(), Default::default()).unwrap();

    // to test
    let command = Command::new("./out.exe").output().unwrap();
    println!("Exit code: {}", command.status.code().unwrap());
    println!("Stdout: {}", str::from_utf8(&command.stdout).unwrap_or_default());
    println!("Stderr: {}", str::from_utf8(&command.stderr).unwrap_or_default());
}
