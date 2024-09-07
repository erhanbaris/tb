use core::str;
use std::env::temp_dir;
use std::{fs::File, io::Write, process::Command};

use tb_builder::ApplicationType;
use tb_core::compiler::TBCompiler;
use tb_core::compiler::CompilerTrait;
use tb_target_x86_64::generator::X86AssemblyGenerator;

mod return_test;
mod add_test;
mod sub_test;
mod div_test;
mod mod_test;
mod mul_test;
mod bitwise_not_test;
mod bitwise_neg_test;
mod bitwise_and_test;
mod bitwise_or_test;
mod bitwise_xor_test;
mod shift_left_test;
mod shift_right_test;
mod inc_test;
mod dec_test;
mod if_test;
mod func_call;

pub fn get_exit_code(application: ApplicationType, file_name: &str, exit_code: i32) {
    let mut source_file_name = temp_dir();
    let mut executable_name = temp_dir();

    source_file_name.push(format!("{}.s", &file_name));
    executable_name.push(format!("{}.exe", &file_name));

    println!("Source: {}", source_file_name.display());
    println!("Exe: {}", executable_name.display());

    let buffer = application.build::<X86AssemblyGenerator>();
    let mut file = File::create(&source_file_name).unwrap();
    file.write_all(buffer.as_bytes()).unwrap();

    let compiler = TBCompiler::get_compiler().unwrap();
    compiler.compile(&source_file_name, &executable_name, Default::default()).unwrap();

    // to test
    let command = Command::new(executable_name).output().unwrap();
    assert_eq!(exit_code, command.status.code().unwrap());
}

pub fn check_output(application: ApplicationType, file_name: &str, expected_message: &str) {
    let mut source_file_name = temp_dir();
    let mut executable_name = temp_dir();

    source_file_name.push(format!("{}.s", &file_name));
    executable_name.push(format!("{}.exe", &file_name));

    println!("Source: {}", source_file_name.display());
    println!("Exe: {}", executable_name.display());

    let buffer = application.build::<X86AssemblyGenerator>();
    let mut file = File::create(&source_file_name).unwrap();
    file.write_all(buffer.as_bytes()).unwrap();

    let compiler = TBCompiler::get_compiler().unwrap();
    compiler.compile(&source_file_name, &executable_name, Default::default()).unwrap();

    // to test
    let command = Command::new(executable_name).output().unwrap();
    assert_eq!(expected_message, str::from_utf8(&command.stdout).unwrap());
}
