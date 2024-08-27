pub trait OsSpecificDefs {
    fn main_function_name(&self) -> &'static str;
    fn end_of_file_instructions(&self) -> &'static str;
}

#[derive(Debug, Clone, Default)]
pub struct MacSpecificDefs;

#[derive(Debug, Clone, Default)]
pub struct LinuxSpecificDefs;


impl OsSpecificDefs for MacSpecificDefs {
    fn main_function_name(&self) -> &'static str {
        "_main"
    }

    fn end_of_file_instructions(&self) -> &'static str {
        ""
    }
}

impl OsSpecificDefs for LinuxSpecificDefs {
    fn main_function_name(&self) -> &'static str {
        "main"
    }

    fn end_of_file_instructions(&self) -> &'static str {
        ".ident	\"TB v0.1.0\""
    }
}

pub fn os_defs() -> Box<dyn OsSpecificDefs> {
    match os_version::detect().unwrap() {
        os_version::OsVersion::Linux(_) => Box::new(LinuxSpecificDefs::default()),
        os_version::OsVersion::MacOS(_) => Box::new(MacSpecificDefs::default()),
        os_version::OsVersion::Windows(_) => Box::new(LinuxSpecificDefs::default()),
        os => panic!("Unsupported OS ({:?})", os)
    }
}
