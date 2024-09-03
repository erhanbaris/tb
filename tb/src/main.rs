use std::{fs::File, io::Write, path::PathBuf, process::Command, str::FromStr};

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use tb_builder::{ApplicationType, BlockType, ConditionType, ExpressionType, FunctionType, IfBlockType};
use tb_core::{compiler::{CompilerTrait, TBCompiler}, types::Value};
use tb_target_x86_64::generator::X86AssemblyGenerator;

#[cfg(test)]
mod tests;

fn main() {
    let _ = CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]);

    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();
    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(10)));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::eq(Value::Number(10), Value::Number(10)));
    if_condition.set_true_block(if_condition_true_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_number(0);
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);
    let buffer = application_type.build::<X86AssemblyGenerator>();
    // println!("{}", &buffer);

    let mut file = File::create("out.s").unwrap();
    file.write_all(buffer.as_bytes()).unwrap();

    let compiler = TBCompiler::get_compiler().unwrap();
    compiler.compile(&PathBuf::from_str("out.s").unwrap(), &PathBuf::from_str("out.exe").unwrap(), Default::default()).unwrap();

    // to test
    let command = Command::new("./out.exe").output().unwrap();
    println!("Exit code: {}", command.status.code().unwrap());
}
