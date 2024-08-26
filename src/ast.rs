use std::collections::HashMap;

use crate::{backend::{Application, BackendType, Instruction, Location, Number}, register::{AddressingMode, Register}};

#[derive(Debug, Clone)]
struct Scope {
    variables: Vec<String>,
    pub last_assigned_location: Location,
    pub registers: Vec<(Register, bool)>
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            variables: Default::default(),
            last_assigned_location: Location::Register(AddressingMode::Immediate(Register::RAX)),
            registers: vec![(Register::RDX, true), (Register::RCX, true), (Register::R8, true), (Register::R9, true), (Register::RDI, true)]
        }
    }
}

impl Scope {
    pub fn find_variable(&self, variable: &str) -> Option<usize> {
        self.variables
            .iter()
            .position(|item| item == variable)
            .map(|item| item + 1)
    }

    pub fn register_backup(&self) -> Vec<(Register, bool)> {
        self.registers.clone()
    }

    pub fn mark_register(&mut self, register: Register) {
        for (index, (register, status)) in self.registers.iter().enumerate() {
            if *status {
                let register = *register;
                self.registers[index] = (register, false);
                break;
            }
        }
    }

    pub fn unmark_register(&mut self, register: Register) {
        for (index, (register, status)) in self.registers.iter().enumerate() {
            if *status {
                let register = *register;
                self.registers[index] = (register, true);
                break;
            }
        }
    }

    pub fn register_restore(&mut self, registers: Vec<(Register, bool)>) {
        self.registers = registers;
    }

    pub fn add_variable(&mut self, name :&str) -> usize {
        self.variables.push(name.to_owned());
        self.variables.len()
    }

    pub fn add_temp_variable(&mut self) -> usize {
        self.variables.push(format!(".t{}", self.variables.len()));
        self.variables.len()
    }

    pub fn lock_register(&mut self) -> Option<Register> {
        for (index, (register, status)) in self.registers.iter().enumerate() {
            if *status {
                let register = *register;
                self.registers[index] = (register, false);
                return Some(register)
            }
        }
        None
    }
    
    pub fn release_register(&mut self, register: Register) {
        if let Some(position) = self.registers.iter().position(|item| item.0 == register) {
            self.registers[position] = (register, true);
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariableType {
    Variable(String),
    Number(i32),
}

impl VariableType {
    pub fn generate(&self, instructions: &mut Vec<Instruction>, scope: &mut Scope) -> Location {
        match self {
            VariableType::Variable(variable) => match scope.find_variable(variable) {
                Some(position) => Location::Register(AddressingMode::create_based(position as i32 * -4, Register::RBP)),
                None => panic!("variable not found")
            },
            VariableType::Number(num) => {
                let position = scope.add_temp_variable();
                let stack = Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP));

                instructions.push(Instruction::Mov { source: Location::Imm(Number::I32(*num)), target: stack.clone(), comment: None });

                let register = scope.lock_register().unwrap();
                instructions.push(Instruction::Mov { source: stack.clone(), target: Location::Register(AddressingMode::Immediate(register)), comment: None });
                Location::Register(AddressingMode::Immediate(register))
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Add {
        target: Box<VariableType>,
        source: Box<VariableType>
    },
    Sub {
        target: Box<VariableType>,
        source: Box<VariableType>
    },
    Mul {
        target: Box<VariableType>,
        source: Box<VariableType>
    },
    Div {
        target: Box<VariableType>,
        source: Box<VariableType>
    },
    Neg {
        target: Box<VariableType>,
        source: Box<VariableType>
    },
    Value(VariableType)
}

impl ExpressionType {
    pub fn generate(&self, scope: &mut Scope) -> Vec<Instruction> {
        match self {
            ExpressionType::Add { target, source } => self.generate_add(scope, target, source),
            ExpressionType::Sub { target, source } => todo!(),
            ExpressionType::Mul { target, source } => todo!(),
            ExpressionType::Div { target, source } => todo!(),
            ExpressionType::Neg { target, source } => todo!(),
            ExpressionType::Value(val) => self.generate_value(val),
        }
    }

    fn generate_add(&self, scope: &mut Scope, target: &VariableType, source: &VariableType) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(Instruction::Comment("Generate source value".to_owned()));
        let mut source = source.generate(&mut instructions, scope);

        instructions.push(Instruction::Comment("Generate target value".to_owned()));
        let target = target.generate(&mut instructions, scope);

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(Instruction::Mov { source, target: Location::Register(AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = Location::Register(AddressingMode::Immediate(new_reg));
            }
        }

        instructions.push(Instruction::Add { source, target: target.clone(), comment: None });
        scope.register_restore(registers);
        scope.last_assigned_location = target.clone();

        if let Some(register) = target.get_register() {
            scope.mark_register(register);
        }

        instructions
    }

    pub fn generate_value(&self, value: &VariableType) -> Vec<Instruction> {
        match value {
            VariableType::Variable(var) => todo!(),
            VariableType::Number(var) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatementType {
    Assign {
        name: String,
        assigne: Box<ExpressionType>
    },

    Return(Option<VariableType>)
}

impl StatementType {
    pub fn generate(&self, scope: &mut Scope) -> Vec<Instruction> {
        match self {
            StatementType::Assign { name, assigne } => self.generate_assign(scope, name, assigne),
            StatementType::Return(expr) => self.generate_return(scope, expr),
        }
    }
    
    pub fn generate_assign(&self, scope: &mut Scope, name: &String, assigne: &Box<ExpressionType>) -> Vec<Instruction> {
        let position = match scope.find_variable(&name) {
            Some(index) => index,
            None => {
                scope.variables.push(name.to_owned());
                scope.variables.len()
            }
        };

        let registers = scope.register_backup();

        let mut instructions = assigne.generate(scope);

        if let Some(mode) = scope.last_assigned_location.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(Instruction::Mov { source: scope.last_assigned_location, target: Location::Register(AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                scope.last_assigned_location = Location::Register(AddressingMode::Immediate(new_reg));
            }
        }

        instructions.push(Instruction::Mov { source: scope.last_assigned_location.clone(), target: Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP)), comment: Some(format!("assign {}", name)) });
        scope.last_assigned_location = Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP));
        scope.register_restore(registers);
        instructions
    }

    pub fn generate_return(&self, scope: &mut Scope, expr: &Option<VariableType>) -> Vec<Instruction> {
        match expr {
            Some(VariableType::Variable(variable)) => {
                if let Some(position) = scope.find_variable(variable) {
                    return vec![Instruction::Mov { source: scope.last_assigned_location.clone(), target: Location::Register(AddressingMode::Immediate(Register::RAX)), comment: Some(format!("return {}", variable)) }]
                }
                Vec::new()
            },
            Some(VariableType::Number(variable)) => vec![Instruction::Mov { source: Location::Imm(Number::I32(*variable)), target: Location::Register(AddressingMode::Immediate(Register::RAX)), comment: Some(format!("return {}", variable)) }],
            None => Vec::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum DefinitionType {
    Function {
        name: String,
        parameters: Vec<Box<VariableType>>,
        body: Vec<Box<StatementType>>
    }
}

impl DefinitionType {
    fn generate(&self) -> BackendType {
        match self {
            DefinitionType::Function { name, parameters, body } => self.generate_function(name, parameters, body)
        }
    }

    fn generate_function(&self, name: &String, parameters: &Vec<Box<VariableType>>, body: &Vec<Box<StatementType>>) -> BackendType {
        let mut instructions = Vec::new();
        let mut scope = Scope::default();
        for item in body.iter() {
            instructions.append(&mut item.generate(&mut scope));
        }

        BackendType::Function { name: name.clone(), instructions }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AstApplication {
    pub asts: Vec<Box<DefinitionType>>
}

#[derive(Debug, Clone, Default)]
pub struct AstApplicationContext {
} 

pub trait BackendGenerate {
    fn generate(&self, context: &mut AstApplicationContext, items: &mut Vec<Instruction>);
}

impl AstApplication {
    pub fn generate(&self) -> Application {
        let mut application = Application::default();
        for item in self.asts.iter() {
            application.items.push(item.generate());
        }
        application
    }
}