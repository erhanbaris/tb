#[derive(Debug)]
pub enum X86Error {
    VariableNotFound(String),
    UnexpectedInstruction,
    NoRegisterAvailable
}
