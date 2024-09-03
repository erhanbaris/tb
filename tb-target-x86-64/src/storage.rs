use tb_core::instruction::StorageTrait;

#[derive(Debug, Default)]
pub struct X86Storage {
    pub branch_counter: usize
}

impl X86Storage {
    pub fn get_branch(&mut self) -> String {
        let name = format!(".L{}", self.branch_counter);
        self.branch_counter += 1;
        name
    }
}

impl StorageTrait for X86Storage {

}
