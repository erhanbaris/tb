use std::{borrow::Borrow, fmt::{Debug, Display}};

use strum_macros::EnumDiscriminants;

use crate::{register::{get_register_type, AddressingMode, Register}, tool::{os_defs, OsSpecificDefs}};

pub trait AsmGenerate {
    fn generate(&self, context: &mut ApplicationContext, buffer: &mut String);
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
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Location {
    Memory(i64),
    Register(AddressingMode),
    Imm(Number)
}

impl Location {
    pub fn get_register(&self) -> Option<Register> {
        match self {
            Location::Register(AddressingMode::Immediate(register)) => Some(*register),
            Location::Register(AddressingMode::Indirect(register)) => Some(*register),
            Location::Register(AddressingMode::Based(_, register)) => Some(*register),
            Location::Register(AddressingMode::Complex) => Some(Register::RAX),
            _ => None
        }
    }
    
    pub fn get_addressing_mode(&self) -> Option<AddressingMode> {
        match self {
            Location::Register(addressing_mode) => Some(*addressing_mode),
            _ => None
        }
    }
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(InstructionType))]
pub enum Instruction {
    Add {
        source: Location,
        target: Location,
        comment: Option<String>
    },
    Not {
        source: Location,
        comment: Option<String>
    },
    Neg {
        source: Location,
        comment: Option<String>
    },
    Mov {
        source: Location,
        target: Location,
        comment: Option<String>
    },

    Push(AddressingMode),
    Pop(AddressingMode),
    Comment(String),
    Ret
}

#[derive(Debug, Clone)]
pub enum Backend {
    Function { name: String, instructions: Vec<Instruction> },
    Instruction(Instruction)
}

impl AsmGenerate for Backend {
    fn generate(&self, context: &mut ApplicationContext, buffer: &mut String) {
        match self {
            Backend::Function { name, instructions } => self.generate_function(context, buffer, name, instructions),
            Backend::Instruction(inst) => self.generate_instruction(context, buffer, inst),
        };
    }
}

impl Backend {
    fn generate_function(&self, context: &mut ApplicationContext, buffer: &mut String, name: &String, instructions: &Vec<Instruction>) {
        buffer.push_str(name);
        buffer.push(':');
        buffer.push_str("\r\n");

        // Function begin
        self.print_inst(Instruction::Push(AddressingMode::Immediate(Register::RBP)), context, buffer);
        self.print_inst(Instruction::Mov { source: Location::Register(AddressingMode::Immediate(Register::RSP)), target: Location::Register(AddressingMode::Immediate(Register::RBP)), comment: None }, context, buffer);

        buffer.push_str("    # function body begin\r\n");

        for instruction in instructions.iter() {
            self.print_inst(instruction, context, buffer);
        }
        buffer.push_str("    # function body end\r\n");

        // Function end
        self.print_inst(Instruction::Mov { source: Location::Register(AddressingMode::Immediate(Register::RBP)), target: Location::Register(AddressingMode::Immediate(Register::RSP)), comment: None }, context, buffer);
        self.print_inst(Instruction::Pop(AddressingMode::Immediate(Register::RBP)), context, buffer);
        self.print_inst(Instruction::Ret, context, buffer);
    }

    fn generate_instruction(&self, _: &mut ApplicationContext, buffer: &mut String, inst: &Instruction) {
        match inst {
            Instruction::Add { source, target, comment } => self.do_add(source, target, comment, buffer),
            Instruction::Not { source, comment } => self.do_not(source, comment, buffer),
            Instruction::Neg { source, comment } => self.do_neg(source, comment, buffer),
            Instruction::Mov { source, target, comment } => self.do_mov(source, target, comment, buffer),
            Instruction::Ret => self.do_ret(buffer),
            Instruction::Push(register) => self.do_push(register, buffer),
            Instruction::Pop(register) => self.do_pop(register, buffer),
            Instruction::Comment(comment) => self.do_comment(comment, buffer)
        };
        buffer.push_str("\r\n");
    }

    fn do_ret(&self, buffer: &mut String) {
        buffer.push_str("ret");
    }
    
    fn do_push(&self, register: &AddressingMode, buffer: &mut String) {
        buffer.push_str(&format!("push{} {}", self.get_suffix(register), register.to_string().to_lowercase()));
    }
    
    fn do_pop(&self, register: &AddressingMode, buffer: &mut String) {
        buffer.push_str(&format!("pop{} {}", self.get_suffix(register), register.to_string().to_lowercase()));
    }
    
    fn do_comment(&self, comment: &String, buffer: &mut String) {
        buffer.push_str(&self.get_comment(&Some(comment.to_owned())));
    }
    
    fn do_add(&self, source: &Location, target: &Location, comment: &Option<String>, buffer: &mut String) {
        match (source, target) {
            (Location::Imm(imm), Location::Register(register)) => buffer.push_str(&format!("add{} ${}, {} {}", self.get_suffix(register), imm, register.to_string().to_lowercase(), self.get_comment(comment))),
            (Location::Register(source_reg), Location::Register(target_reg)) => buffer.push_str(&format!("add{} {}, {} {}", self.get_suffix_from_registers(source_reg, target_reg), source_reg.to_string().to_lowercase(), target_reg.to_string().to_lowercase(), self.get_comment(comment))),
            value => panic!("unsupported ({:?})", value)
        }
    }
    
    fn do_not(&self, source: &Location, comment: &Option<String>, buffer: &mut String) {
        match source {
            Location::Register(source_reg) => buffer.push_str(&format!("not {} {}", source_reg.to_string().to_lowercase(), self.get_comment(comment))),
            _ => panic!("unsupported")
        }
    }
    
    fn do_neg(&self, source: &Location, comment: &Option<String>, buffer: &mut String) {
        match source {
            Location::Register(source_reg) => buffer.push_str(&format!("neg {} {}", source_reg.to_string().to_lowercase(), self.get_comment(comment))),
            _ => panic!("unsupported")
        }
    }
    
    fn do_mov(&self, source: &Location, target: &Location, comment: &Option<String>, buffer: &mut String) {
        match (source, target) {
            (Location::Imm(imm), Location::Register(register)) => buffer.push_str(&format!("mov{} ${}, {} {}", self.get_suffix(register), imm, register.to_string().to_lowercase(), self.get_comment(comment))),
            (Location::Register(source_reg), Location::Register(target_reg)) => buffer.push_str(&format!("mov{} {}, {} {}", self.get_suffix_from_registers(source_reg, target_reg), source_reg.to_string().to_lowercase(), target_reg.to_string().to_lowercase(), self.get_comment(comment))),
            _ => panic!("unsupported")
        }
    }

    fn print_inst<T: Borrow<Instruction>>(&self, inst: T, context: &mut ApplicationContext, buffer: &mut String) {
        buffer.push_str("    ");
        self.generate_instruction(context, buffer, inst.borrow());
    }

    fn get_comment(&self, comment: &Option<String>) -> String {
        match comment {
            Some(comment) => format!("# {}", comment),
            None => String::new()
        }
    }

    fn get_suffix(&self, mode: &AddressingMode) -> &str {
        match mode {
            AddressingMode::Immediate(_) => "",
            AddressingMode::Indirect(_) => "q",
            AddressingMode::Based(_, _) => "q",
            AddressingMode::Complex => "q",
        }
    }

    fn get_suffix_from_registers(&self, mode1: &AddressingMode, mode2: &AddressingMode) -> &str {
        let mode1_register = mode1.get_register();
        let mode2_register = mode2.get_register();

        let mode1_register_type = get_register_type(mode1_register);
        let mode2_register_type = get_register_type(mode2_register);

        match mode1_register_type != mode2_register_type {
            true => "l",
            false => ""
        }
    }
}

pub struct ApplicationContext {
    pub os_specific_defs: Box<dyn OsSpecificDefs>
}

impl Default for ApplicationContext {
    fn default() -> Self {
        Self {
            os_specific_defs: os_defs()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Application {
    pub items: Vec<Backend>
}

impl AsmGenerate for Application {
    fn generate(&self, context: &mut ApplicationContext, buffer: &mut String) {
        buffer.push_str(&format!(".globl {}\r\n", context.os_specific_defs.main_function_name()));
        for func in self.items.iter() {
            func.generate(context, buffer);
        }

        buffer.push_str(context.os_specific_defs.end_of_file_instructions());
    }
}
