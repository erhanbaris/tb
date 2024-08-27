use std::os::raw::c_char;

pub type FunctionType = tb_builder::FunctionType;

// A Rust struct mapping the C struct
#[repr(C)]
#[derive(Debug)]
pub struct RustStruct {
    pub c: char,
    pub ul: u64,
    pub c_string: *const c_char,
}
