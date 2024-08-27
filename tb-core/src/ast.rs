
use strum_macros::EnumDiscriminants;

use crate::{backend::{Application, Backend, Instruction, Location, Number}, register::{AddressingMode, Register}};

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
pub enum Variable {
    Variable(String),
    Number(i32),
}

impl Variable {
    fn generate(&self, instructions: &mut Vec<Instruction>, scope: &mut Scope) -> Location {
        match self {
            Variable::Variable(variable) => match scope.find_variable(variable) {
                Some(position) => Location::Register(AddressingMode::create_based(position as i32 * -4, Register::RBP)),
                None => panic!("variable not found")
            },
            Variable::Number(num) => {
                let position = scope.add_temp_variable();
                let stack = Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP));

                instructions.push(Instruction::Mov { source: Location::Imm(Number::I32(*num)), target: stack, comment: None });

                let register = scope.lock_register().unwrap();
                instructions.push(Instruction::Mov { source: stack, target: Location::Register(AddressingMode::Immediate(register)), comment: None });
                Location::Register(AddressingMode::Immediate(register))
            },
        }
    }
}

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(ExpressionDiscriminant))]
pub enum Expression {
    Add {
        target: Variable,
        source: Variable
    },
    Not {
        source: Variable
    },
    Neg {
        source: Variable
    },
    Value(Variable)
}

impl Expression {
    fn generate(&self, scope: &mut Scope) -> Vec<Instruction> {
        match self {
            Expression::Add { target, source } => self.generate_add(scope, target, source),
            Expression::Not { source } => self.generate_not(scope, source),
            Expression::Neg { source } => self.generate_neg(scope, source),
            Expression::Value(val) => self.generate_value(val),
        }
    }

    fn generate_add(&self, scope: &mut Scope, target: &Variable, source: &Variable) -> Vec<Instruction> {
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

        instructions.push(Instruction::Add { source, target, comment: None });
        scope.register_restore(registers);
        scope.last_assigned_location = target;

        if let Some(register) = target.get_register() {
            scope.mark_register(register);
        }

        instructions
    }

    fn generate_not(&self, scope: &mut Scope, source: &Variable) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(Instruction::Comment("Generate source value".to_owned()));
        let mut source = source.generate(&mut instructions, scope);

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(Instruction::Mov { source, target: Location::Register(AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = Location::Register(AddressingMode::Immediate(new_reg));
            }
        }

        instructions.push(Instruction::Not { source, comment: None });
        scope.register_restore(registers);
        scope.last_assigned_location = source;

        if let Some(register) = source.get_register() {
            scope.mark_register(register);
        }

        instructions
    }

    fn generate_neg(&self, scope: &mut Scope, source: &Variable) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let registers = scope.register_backup();

        instructions.push(Instruction::Comment("Generate source value".to_owned()));
        let mut source = source.generate(&mut instructions, scope);

        if let Some(mode) = source.get_addressing_mode() {
            if !mode.is_direct_register() {
                let new_reg = scope.lock_register().unwrap();
                instructions.push(Instruction::Mov { source, target: Location::Register(AddressingMode::Immediate(new_reg)), comment: Some("Move address to reg for calculation".to_owned()) });
                source = Location::Register(AddressingMode::Immediate(new_reg));
            }
        }

        instructions.push(Instruction::Neg { source, comment: None });
        scope.register_restore(registers);
        scope.last_assigned_location = source;

        if let Some(register) = source.get_register() {
            scope.mark_register(register);
        }

        instructions
    }

    pub fn generate_value(&self, value: &Variable) -> Vec<Instruction> {
        match value {
            Variable::Variable(var) => todo!(),
            Variable::Number(var) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assign {
        name: String,
        assigne: Box<Expression>
    },

    Return(Option<Variable>)
}

impl Statement {
    fn generate(&self, scope: &mut Scope) -> Vec<Instruction> {
        match self {
            Statement::Assign { name, assigne } => self.generate_assign(scope, name, assigne),
            Statement::Return(expr) => self.generate_return(scope, expr),
        }
    }
    
    fn generate_assign(&self, scope: &mut Scope, name: &String, assigne: &Box<Expression>) -> Vec<Instruction> {
        let position = match scope.find_variable(name) {
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

        instructions.push(Instruction::Mov { source: scope.last_assigned_location, target: Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP)), comment: Some(format!("assign {}", name)) });
        scope.last_assigned_location = Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP));
        scope.register_restore(registers);
        instructions
    }

    fn generate_return(&self, scope: &mut Scope, expr: &Option<Variable>) -> Vec<Instruction> {
        match expr {
            Some(Variable::Variable(variable)) => {
                if let Some(position) = scope.find_variable(variable) {
                    return vec![Instruction::Mov { source: scope.last_assigned_location, target: Location::Register(AddressingMode::Immediate(Register::RAX)), comment: Some(format!("return {}", variable)) }]
                }
                Vec::new()
            },
            Some(Variable::Number(variable)) => vec![Instruction::Mov { source: Location::Imm(Number::I32(*variable)), target: Location::Register(AddressingMode::Immediate(Register::RAX)), comment: Some(format!("return {}", variable)) }],
            None => Vec::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Definition {
    Function {
        name: String,
        parameters: Vec<Box<Variable>>,
        body: Vec<Box<Statement>>
    }
}

impl Definition {
    fn generate(&self) -> Backend {
        match self {
            Definition::Function { name, parameters, body } => self.generate_function(name, parameters, body)
        }
    }

    fn generate_function(&self, name: &String, parameters: &Vec<Box<Variable>>, body: &Vec<Box<Statement>>) -> Backend {
        let mut instructions = Vec::new();
        let mut scope = Scope::default();
        for item in body.iter() {
            instructions.append(&mut item.generate(&mut scope));
        }

        Backend::Function { name: name.clone(), instructions }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AstApplication {
    pub asts: Vec<Box<Definition>>
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