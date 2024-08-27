#![deny(improper_ctypes_definitions)]

use std::os::raw::c_int;

/// Analyze the numbers.
#[no_mangle]
pub extern "C" fn analyze_numbers(x: c_int, y: c_int) {
    if x < y {
        println!("x ({x}) is smallest!");
    } else {
        println!("y ({y}) is probably larger than x ({x})");
    }
}
