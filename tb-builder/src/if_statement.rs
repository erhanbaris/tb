use tb_core::types::Value;

use crate::BlockType;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IfConditionType {
    name: String,
    parameters: Vec<Value>,
    block: BlockType
}
