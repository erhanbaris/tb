use tb_core::types::{ApplicationContext, AssemblyGenerator, Definition};

use super::{BuilderGenerate, FunctionType};

#[derive(Debug, Clone, Default)]
pub struct ApplicationType {
    definitions: Vec<Definition>
}

impl ApplicationType {
    pub fn add_function(&mut self, func: FunctionType) {
        self.definitions.push(func.convert());
    }

    pub fn build<A: AssemblyGenerator>(self) -> String {
        let application = A::default();
        let context = ApplicationContext::default();
        application.generate(self.definitions.clone(), context)
    }
}
