
use std::fmt::{Debug, Display};

use strum_macros::EnumDiscriminants;

use crate::{instruction::{InstructionTrait, StorageTrait}, syntax::AsmStructure, tool::{os_defs, OsSpecificDefs}};

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
    ShiftLeft {
        target: Value,
        source: Value
    },
    ShiftRight {
        target: Value,
        source: Value
    },
    BitwiseNot {
        source: Value
    },
    BitwiseAnd {
        target: Value,
        source: Value
    },
    BitwiseOr {
        target: Value,
        source: Value
    },
    BitwiseXor {
        target: Value,
        source: Value
    },
    BitwiseNeg {
        source: Value
    },
    Inc {
        source: Value
    },
    Dec {
        source: Value
    },
    Value(Value)
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(ConditionDiscriminant))]
pub enum Condition {
    Eq {
        left: Value,
        right: Value
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assign {
        name: String,
        assigne: Expression
    },
    If {
        condition: Condition,
        true_block: Block,
        false_block: Option<Block>
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


#[derive(Debug)]
pub struct InstructionCollection<I: InstructionTrait> {
    pub items: Vec<AsmStructure<I>>,
}

impl<I> Default for InstructionCollection<I> where I: InstructionTrait {
    fn default() -> Self {
        Self {
            items: Default::default()
        }
    }
}

impl<I> InstructionCollection<I> where I: InstructionTrait {
    pub fn add_instruction(&mut self, instruction: I) {
        self.items.push(AsmStructure::Instruction(Box::new(instruction)))
    }

    pub fn add_branch(&mut self, name: String) {
        self.items.push(AsmStructure::Branch(name))
    }

    pub fn add_close_branch(&mut self) {
        self.items.push(AsmStructure::BranchFinished)
    }
    
    pub fn add_comment(&mut self, comment: String) {
        self.items.push(AsmStructure::Comment(comment))
    }
}

pub struct ApplicationContext<I: InstructionTrait, S: StorageTrait> {
    pub os_specific_defs: Box<dyn OsSpecificDefs>,
    pub instructions: InstructionCollection<I>,
    pub storage: S
}

impl<I, S> Default for ApplicationContext<I, S> where I: InstructionTrait, S: StorageTrait {
    fn default() -> Self {
        Self {
            os_specific_defs: os_defs(),
            storage: Default::default(),
            instructions: Default::default()
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

#[derive(Ord, Eq, PartialOrd, Debug, Copy, Clone, PartialEq)]
pub enum RegisterSize {
    _8Bit = 0,
    _16Bit = 1,
    _32Bit = 2,
    _64Bit = 3
}

pub trait RegisterTrait: Clone + PartialEq + Debug + ToString {
    fn get_register_size(&self) -> RegisterSize;
}
