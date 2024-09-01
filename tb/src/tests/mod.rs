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
