use std::fmt::Debug;

use tb_core::types::{AssemblyGenerator, DataItemCollection, Definition};

use super::{BuilderGenerate, FunctionType};

#[derive(Debug, Clone, Default)]
pub struct ApplicationType {
    definitions: Vec<Definition>,
    datas: DataItemCollection,
}

impl ApplicationType {
    pub fn add_function(&mut self, func: FunctionType) {
        self.definitions.push(func.convert());
    }

    pub fn add_string_data<L: AsRef<str>, D: AsRef<str>>(&mut self, label: L, data: D) {
        self.datas.add_string_data(label, data)
    }

    pub fn add_byte_data<L: AsRef<str>>(&mut self, label: L, data: u8) {
        self.datas.add_byte_data(label, data)
    }

    pub fn build<A: AssemblyGenerator>(self) -> String {
        let Self { definitions, datas } = self;

        let application = A::default();
        application.generate(definitions, datas)
    }
}
