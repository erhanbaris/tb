use std::marker::PhantomData;

pub trait StoreDefaultRegisters<R: Clone + PartialEq>: Clone {
    fn initialize() -> Vec<(R, bool)>;
}

#[derive(Debug, Clone)]
pub struct Store<R: Clone + PartialEq, L: Clone, D: StoreDefaultRegisters<R> + Clone> {
    variables: Vec<String>,
    last_assigned_location: L,
    registers: Vec<(R, bool)>,
    _mark: PhantomData<D>
}

impl<R, L, D> Default for Store<R, L, D>
where
    R: Clone + PartialEq,
    L: Default + Clone,
    D: StoreDefaultRegisters<R> + Clone
{
    fn default() -> Self {
        Self {
            variables: Default::default(),
            last_assigned_location: L::default(),
            registers: D::initialize(),
            _mark: PhantomData
        }
    }
}

impl<R, L, D> Store<R, L, D>
where
    R: Clone + PartialEq,
    L: Clone,
    D: StoreDefaultRegisters<R> + Clone
{
    pub fn get_last_assigned_location(&mut self) -> L {
        self.last_assigned_location.clone()
    }

    pub fn set_last_assigned_location(&mut self, location: L) {
        self.last_assigned_location = location;
    }

    pub fn find_variable(&self, variable: &str) -> Option<usize> {
        self.variables
            .iter()
            .position(|item| item == variable)
            .map(|item| item + 1)
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

    pub fn add_variable(&mut self, name: &str) -> usize {
        self.variables.push(name.to_owned());
        self.variables.len()
    }

    pub fn add_temp_variable(&mut self) -> usize {
        self.variables.push(format!(".t{}", self.variables.len()));
        self.variables.len()
    }

    pub fn variable_size(&self) -> usize {
        self.variables.len()
    }

    pub fn lock_register(&mut self) -> Option<R> {
        for (index, (register, status)) in self.registers.iter().enumerate() {
            if *status {
                let register = register.clone();
                self.registers[index] = (register.clone(), false);
                return Some(register);
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
