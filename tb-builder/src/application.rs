use tb_core::{ast::{AstApplication, Definition}, backend::{ApplicationContext, AsmGenerate}};

use super::{BuilderGenerate, FunctionType};

#[derive(Debug, Clone, Default)]
pub struct ApplicationType {
    definitions: Vec<Box<Definition>>
}

impl ApplicationType {
    pub fn add_function(&mut self, func: FunctionType) {
        self.definitions.push(Box::new(func.convert()));
    }

    pub fn build(self) -> String {
        let mut application = AstApplication::default();
        let mut context = ApplicationContext::default();
        let mut buffer = String::new();
        
        application.asts = self.definitions;
        let backend_application = application.generate();
        backend_application.generate(&mut context, &mut buffer);
        buffer
    }
}
