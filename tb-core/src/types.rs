
use std::fmt::{Debug, Display};

use strum_macros::EnumDiscriminants;

use crate::{instruction::InstructionTrait, syntax::AsmStructure, tool::{os_defs, OsSpecificDefs}};

#[derive(Debug, Clone)]
pub enum Value {
    Variable(String),
    Number(i64),
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(ExpressionDiscriminant))]
pub enum Expression {
    Add {
        target: Value,
        source: Value
    },
    Sub {
        target: Value,
        source: Value
    },
    Div {
        divider: Value,
        divided: Value
    },
    Mul {
        target: Value,
        source: Value
    },
    Modulo {
        divider: Value,
        divided: Value
    },
    BitwiseNot {
        source: Value
    },
    BitwiseAnd {
        target: Value,
        source: Value
    },
    Neg {
        source: Value
    },
    Value(Value)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assign {
        name: String,
        assigne: Expression
    },
    Return(Option<Value>)
}

#[derive(Debug, Clone)]
pub enum Definition {
    Function {
        name: String,
        parameters: Vec<Value>,
        block: Block
    },
}

#[derive(Debug, Clone)]
pub struct Block {
    pub items: Vec<Statement>
}

pub struct ApplicationContext<I: InstructionTrait> {
    pub os_specific_defs: Box<dyn OsSpecificDefs>,
    pub abstract_asms: Vec<AsmStructure<I>>
}

impl<I> Default for ApplicationContext<I> where I: InstructionTrait {
    fn default() -> Self {
        Self {
            os_specific_defs: os_defs(),
            abstract_asms: Default::default()
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Number {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Float(f32),
    Bool(bool),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::I8(num) => write!(f, "{}", num),
            Number::U8(num) => write!(f, "{}", num),
            Number::I16(num) => write!(f, "{}", num),
            Number::U16(num) => write!(f, "{}", num),
            Number::I32(num) => write!(f, "{}", num),
            Number::U32(num) => write!(f, "{}", num),
            Number::I64(num) => write!(f, "{}", num),
            Number::U64(num) => write!(f, "{}", num),
            Number::Float(num) => write!(f, "{}", num),
            Number::Bool(val) => write!(f, "{}", match val {
                true => 1,
                false => 0
            }),
        }
    }
}

pub trait AssemblyGenerator: Default {
    fn generate(&self, definitions: Vec<Definition>) -> String;
}


#[derive(Copy, Clone, PartialEq)]
pub enum RegisterType {
    _8Bit,
    _16Bit,
    _32Bit,
    _64Bit
}

pub trait RegisterTrait: Clone + PartialEq + Debug + ToString {
    fn get_register_type(&self) -> RegisterType;
}
