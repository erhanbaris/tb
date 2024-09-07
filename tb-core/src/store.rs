use std::marker::PhantomData;

use crate::types::{RegisterSize, RegisterTrait};

pub trait StoreDefaultRegisters<R: RegisterTrait>: Clone {
    fn initialize() -> Vec<(R, bool)>;
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub size: u8,
    pub position: usize
}

#[derive(Debug, Clone)]
pub struct Store<R: RegisterTrait, L: Clone, D: StoreDefaultRegisters<R> + Clone> {
    variables: Vec<Variable>,
    last_position: usize,
    last_assigned_location: L,
    last_size: RegisterSize,
    registers: Vec<(R, bool)>,
    _mark: PhantomData<D>
}

impl<R, L, D> Default for Store<R, L, D>
where
    R: RegisterTrait,
    L: Default + Clone,
    D: StoreDefaultRegisters<R> + Clone
{
    fn default() -> Self {
        Self {
            variables: Default::default(),
            last_position: 0,
            last_assigned_location: L::default(),
            registers: D::initialize(),
            last_size: RegisterSize::_32Bit,
            _mark: PhantomData
        }
    }
}

impl<R, L, D> Store<R, L, D>
where
    R: RegisterTrait,
    L: Clone,
    D: StoreDefaultRegisters<R> + Clone
{
    pub fn get_last_assigned_location(&mut self) -> L {
        self.last_assigned_location.clone()
    }

    pub fn get_last_position(&self) -> usize {
        self.last_position
    }

    pub fn set_last_assigned_location(&mut self, location: L) {
        self.last_assigned_location = location;
    }
    
    pub fn get_last_size(&self) -> RegisterSize {
        self.last_size
    }

    pub fn set_last_size(&mut self, size: RegisterSize) {
        self.last_size = size;
    }

    pub fn find_variable(&self, variable: &str) -> Option<&Variable> {
        self.variables
            .iter()
            .find(|item| item.name == variable)
    }

    pub fn register_backup(&self) -> Vec<(R, bool)> {
        self.registers.clone()
    }

    pub fn is_free(&mut self, register: R) -> bool {
        for (item, status) in self.registers.iter() {
            if item == &register {
                return *status;
            }
        }
        
        true
    }

    pub fn mark_register(&mut self, register: R) {
        for (index, (item, _)) in self.registers.iter().enumerate() {
            if item == &register {
                self.registers[index] = (register, false);
                break;
            }
        }
    }

    pub fn unmark_register(&mut self, register: R) {
        for (index, (item, _)) in self.registers.iter().enumerate() {
            if item == &register {
                self.registers[index] = (register, true);
                break;
            }
        }
    }

    pub fn register_restore(&mut self, registers: Vec<(R, bool)>) {
        self.registers = registers;
    }

    pub fn add_variable(&mut self, name: &str, size: u8) -> &Variable {
        self.variables.push(Variable { name: name.to_owned(), size, position: self.last_position + size as usize });
        self.last_position += size as usize;
        &self.variables[self.variables.len()-1]
    }

    pub fn add_temp_variable(&mut self, size: u8) -> &Variable {
        self.variables.push(Variable { name: format!(".t{}", self.variables.len()), size, position: self.last_position + size as usize });
        self.last_position += size as usize;
        &self.variables[self.variables.len()-1]
    }

    pub fn lock_register(&mut self, num_size: RegisterSize) -> Option<R> {
        for (index, (register, status)) in self.registers.iter().enumerate() {
            if *status {
                let register = register.clone();
                self.registers[index] = (register.clone(), false);
                return Some(register.get_sized(num_size)); // Get register based on size (64: RAX, 32: EAX, 16: AX, 8bit: AL)
            }
        }
        None
    }

    pub fn release_register(&mut self, register: R) {
        if let Some(position) = self.registers.iter().position(|item| item.0 == register) {
            self.registers[position] = (register, true);
        }
    }
}
