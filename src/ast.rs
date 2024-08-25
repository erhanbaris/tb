use crate::backend::{AddressingMode, Application, BackendType, Instruction, Location, Number, Register};

#[derive(Debug, Clone)]
struct Scope {
    pub variables: Vec<String>,
    pub last_assigned_location: Location
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            variables: Default::default(),
            last_assigned_location: Location::Register(AddressingMode::Immediate(Register::RAX))
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariableType {
    Variable(String),
    Number(i32),
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Add {
        left: Box<VariableType>,
        right: Box<VariableType>
    },
    Sub {
        left: Box<VariableType>,
        right: Box<VariableType>
    },
    Mul {
        left: Box<VariableType>,
        right: Box<VariableType>
    },
    Div {
        left: Box<VariableType>,
        right: Box<VariableType>
    },
    Neg {
        left: Box<VariableType>,
        right: Box<VariableType>
    },
    Value(VariableType)
}

impl ExpressionType {
    pub fn generate(&self, scope: &mut Scope) -> Vec<Instruction> {
        match self {
            ExpressionType::Add { left, right } => self.generate_add(scope, left, right),
            ExpressionType::Sub { left, right } => todo!(),
            ExpressionType::Mul { left, right } => todo!(),
            ExpressionType::Div { left, right } => todo!(),
            ExpressionType::Neg { left, right } => todo!(),
            ExpressionType::Value(val) => self.generate_value(val),
        }
    }

    fn generate_add(&self, scope: &mut Scope, left: &VariableType, right: &VariableType) -> Vec<Instruction> {
        let left = match left {
            VariableType::Variable(variable) => match scope.variables.iter().position(|item| item == variable) {
                Some(position) => Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP)),
                None => panic!("variable not found")
            },
            VariableType::Number(num) => Location::Imm(Number::I32(*num)),
        };

        let right = match right {
            VariableType::Variable(variable) => match scope.variables.iter().position(|item| item == variable) {
                Some(position) => Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP)),
                None => panic!("variable not found")
            },
            VariableType::Number(num) => Location::Imm(Number::I32(*num)),
        };

        scope.last_assigned_location = right.clone();
        vec![Instruction::Add { source: left, target: right }]
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
            StatementType::Assign { name, assigne } => self.build_assign(scope, name, assigne),
            StatementType::Return(expr) => self.build_return(scope, expr),
        }
    }
    
    pub fn build_assign(&self, scope: &mut Scope, name: &String, assigne: &Box<ExpressionType>) -> Vec<Instruction> {
        let position = match scope.variables.iter().position(|item| item == name) {
            Some(index) => index,
            None => {
                scope.variables.push(name.to_owned());
                scope.variables.len() - 1
            }
        };

        let mut instructions = assigne.generate(scope);
        instructions.push(Instruction::Mov { source: scope.last_assigned_location.clone(), target: Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP)) });
        instructions
    }

    pub fn build_return(&self, scope: &mut Scope, expr: &Option<VariableType>) -> Vec<Instruction> {
        match expr {
            Some(VariableType::Variable(variable)) => {
                if let Some(position) = scope.variables.iter().position(|item| item == variable) {
                    return vec![Instruction::Mov { source: Location::Register(AddressingMode::Based(position as i32 * -4, Register::RBP)), target: Location::Register(AddressingMode::Immediate(Register::RAX)) }]
                }
                Vec::new()
            },
            Some(VariableType::Number(variable)) => vec![Instruction::Mov { source: Location::Imm(Number::I32(*variable)), target: Location::Register(AddressingMode::Immediate(Register::RAX)) }],
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
            DefinitionType::Function { name, parameters, body } => self.build_function(name, parameters, body)
        }
    }

    fn build_function(&self, name: &String, parameters: &Vec<Box<VariableType>>, body: &Vec<Box<StatementType>>) -> BackendType {
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