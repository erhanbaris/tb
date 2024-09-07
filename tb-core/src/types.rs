
use std::fmt::{Debug, Display};

use strum_macros::EnumDiscriminants;

use crate::{instruction::{InstructionTrait, StorageTrait}, syntax::{AsmStructure, Data, DataItem}, tool::{os_defs, OsSpecificDefs}};

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(ValueType))]
pub enum Value {
    Variable(String),
    Number(Number),
    String(String)
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
    },
    Ne {
        left: Value,
        right: Value
    },
    Gr {
        left: Value,
        right: Value
    },
    Ge {
        left: Value,
        right: Value
    },
    Ls {
        left: Value,
        right: Value
    },
    Le {
        left: Value,
        right: Value
    },
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
    Print {
        format: String,
        argument: Option<Value>
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


#[derive(Debug, Clone, Default)]
pub struct DataItemCollection {
    pub items: Vec<DataItem>,
}

impl DataItemCollection {
    pub fn add_string_data<L: AsRef<str>, D: AsRef<str>>(&mut self, label: L, data: D) {
        match self.items.iter_mut().find(|item| item.label == label.as_ref()) {
            Some(item) => item.values.push(Data::String(data.as_ref().to_owned())),
            None => {
                let data = DataItem { label: label.as_ref().to_owned(), values: vec![Data::String(data.as_ref().to_owned())] };
                self.items.push(data);
            },
        };
    }

    pub fn add_byte_data<L: AsRef<str>>(&mut self, label: L, data: u8) {
        match self.items.iter_mut().find(|item| item.label == label.as_ref()) {
            Some(item) => item.values.push(Data::Byte(data)),
            None => {
                let data = DataItem { label: label.as_ref().to_owned(), values: vec![Data::Byte(data)] };
                self.items.push(data);
            },
        };
    }

    pub fn create_label(&mut self) -> String {
        format!("LC{}", self.items.len() + 1)
    }
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
    pub fn last_instruction_position(&self) -> usize {
        self.items.len() - 1
    }

    pub fn add_instruction(&mut self, instruction: I) -> usize {
        self.items.push(AsmStructure::Instruction(Box::new(instruction)));
        self.items.len() - 1
    }

    pub fn update_instruction(&mut self, instruction: I, position: usize) {
        self.items[position] = AsmStructure::Instruction(Box::new(instruction));
    }

    pub fn remove_instruction(&mut self, position: usize) {
        self.items.remove(position);
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
    pub datas: DataItemCollection,
    pub storage: S
}

impl<I, S> Default for ApplicationContext<I, S> where I: InstructionTrait, S: StorageTrait {
    fn default() -> Self {
        Self {
            os_specific_defs: os_defs(),
            storage: Default::default(),
            datas: Default::default(),
            instructions: Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone, EnumDiscriminants)]
#[strum_discriminants(name(NumberType))]
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
    Double(f64),
    Bool(bool),
}

impl From<i8> for Number { fn from(value:  i8) -> Self { Number::I8(value) } }
impl From<u8> for Number { fn from(value:  u8) -> Self { Number::U8(value) } }
impl From<i16> for Number { fn from(value: i16) -> Self { Number::I16(value) } }
impl From<u16> for Number { fn from(value: u16) -> Self { Number::U16(value) } }
impl From<i32> for Number { fn from(value: i32) -> Self { Number::I32(value) } }
impl From<u32> for Number { fn from(value: u32) -> Self { Number::U32(value) } }
impl From<i64> for Number { fn from(value: i64) -> Self { Number::I64(value) } }
impl From<u64> for Number { fn from(value: u64) -> Self { Number::U64(value) } }
impl From<f32> for Number { fn from(value: f32) -> Self { Number::Float(value) } }
impl From<f64> for Number { fn from(value: f64) -> Self { Number::Double(value) } }
impl From<bool> for Number { fn from(value: bool) -> Self { Number::Bool(value) } }

impl Number {
    pub fn size(&self) -> RegisterSize {
        match self {
            Number::I8(_) => RegisterSize::_8Bit,
            Number::U8(_) => RegisterSize::_8Bit,
            Number::I16(_) => RegisterSize::_16Bit,
            Number::U16(_) => RegisterSize::_16Bit,
            Number::I32(_) => RegisterSize::_32Bit,
            Number::U32(_) => RegisterSize::_32Bit,
            Number::I64(_) => RegisterSize::_64Bit,
            Number::U64(_) => RegisterSize::_64Bit,
            Number::Float(_) => RegisterSize::_32Bit,
            Number::Double(_) => RegisterSize::_64Bit,
            Number::Bool(_) => RegisterSize::_8Bit,
        }
    }
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
            Number::Double(num) => write!(f, "{}", num),
            Number::Bool(val) => write!(f, "{}", match val {
                true => 1,
                false => 0
            }),
        }
    }
}

pub trait AssemblyGenerator: Default {
    fn generate(&self, definitions: Vec<Definition>, datas: DataItemCollection) -> String;
}

#[derive(Ord, Eq, PartialOrd, Debug, Copy, Clone, PartialEq)]
pub enum RegisterSize {
    _8Bit = 0,
    _16Bit = 1,
    _32Bit = 2,
    _64Bit = 3
}

impl From<u8> for RegisterSize {
    fn from(value: u8) -> Self {
        match value {
            1 => RegisterSize::_8Bit,
            2 => RegisterSize::_16Bit,
            4 => RegisterSize::_32Bit,
            _ => RegisterSize::_64Bit
        }
    }
}

pub trait RegisterTrait: Clone + PartialEq + Debug + ToString {
    fn get_register_size(&self) -> RegisterSize;
    fn get_sized(self, size: RegisterSize) -> Self;
}
