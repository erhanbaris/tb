pub trait OsSpecificDefs {
    fn main_function_name(&self) -> &'static str;
    fn end_of_file_instructions(&self) -> &'static str;
    fn print(&self) -> &'static str;
    fn readonly_string_section(&self) -> &'static str;
}

#[derive(Debug, Clone, Default)]
pub struct MacSpecificDefs;

#[derive(Debug, Clone, Default)]
pub struct LinuxSpecificDefs;

#[derive(Debug, Clone, Default)]
pub struct WindowsSpecificDefs;


impl OsSpecificDefs for MacSpecificDefs {
    fn main_function_name(&self) -> &'static str {
        "_main"
    }

    fn end_of_file_instructions(&self) -> &'static str {
        ""
    }
    
    fn print(&self) -> &'static str {
        "_printf"
    }
    
    fn readonly_string_section(&self) -> &'static str {
        ".section    __TEXT,__cstring,cstring_literals"
    }
}

impl OsSpecificDefs for LinuxSpecificDefs {
    fn main_function_name(&self) -> &'static str {
        "main"
    }

    fn end_of_file_instructions(&self) -> &'static str {
        ""
    }
    
    fn print(&self) -> &'static str {
        "printf"
    }
    
    fn readonly_string_section(&self) -> &'static str {
        ".text\r\n.section	.rodata"
    }
}

impl OsSpecificDefs for WindowsSpecificDefs {
    fn main_function_name(&self) -> &'static str {
        "main"
    }

    fn end_of_file_instructions(&self) -> &'static str {
        ""
    }
    
    fn print(&self) -> &'static str {
        "_printf"
    }
    
    fn readonly_string_section(&self) -> &'static str {
        ".section    __TEXT,__cstring,cstring_literals"
    }
}

pub fn os_defs() -> Box<dyn OsSpecificDefs> {
    match os_version::detect().unwrap() {
        os_version::OsVersion::Linux(_) => Box::new(LinuxSpecificDefs),
        os_version::OsVersion::MacOS(_) => Box::new(MacSpecificDefs),
        os_version::OsVersion::Windows(_) => Box::new(WindowsSpecificDefs),
        os => panic!("Unsupported OS ({:?})", os)
    }
}
