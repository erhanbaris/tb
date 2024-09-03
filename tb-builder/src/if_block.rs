use tb_core::types::Statement;

use crate::{BlockType, ConditionType};

use super::BuilderGenerate;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IfBlockType {
    pub condition: ConditionType,
    pub true_block: BlockType,
    pub false_block: Option<BlockType>
}

impl IfBlockType {
    pub fn set_condition(&mut self, condition: ConditionType) {
        self.condition = condition;
    }

    pub fn set_true_block(&mut self, true_block: BlockType) {
        self.true_block = true_block;
    }

    pub fn set_false_block(&mut self, false_block: BlockType) {
        self.false_block = Some(false_block);
    }
}

impl BuilderGenerate for IfBlockType {
    type Output = Statement;

    fn convert(self) -> Self::Output {
        let Self { condition, true_block, false_block } = self;
        Statement::If { condition: condition.convert(), true_block: true_block.convert(), false_block: false_block.map(|item| item.convert()) }
    }
}
