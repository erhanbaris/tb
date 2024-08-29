use std::{fs::File, io::Write, process::Command};

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use tb_builder::{ApplicationType, ExpressionType, FunctionType};
use tb_core::{compiler::{CompilerTrait, TBCompiler}, types::Value};
use tb_target_x86_64::generator::X86AssemblyGenerator;

fn main() {
    let _ = CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]);

    let mut main_func = FunctionType::main();
    main_func.add_assign("test1", ExpressionType::sub(Value::Number(1), Value::Number(10)));
    //main_func.add_return_number(55);
    main_func.add_return_variable("test1");

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);
    let buffer = application_type.build::<X86AssemblyGenerator>();
    // println!("{}", &buffer);

    let mut file = File::create("out.s").unwrap();
    file.write_all(buffer.as_bytes()).unwrap();

    let compiler = TBCompiler::get_compiler().unwrap();
    compiler.compile("out.s", "out.exe", Default::default()).unwrap();

    // to test
    let command = Command::new("./out.exe").output().unwrap();
    println!("Exit code: {}", command.status.code().unwrap());
}
