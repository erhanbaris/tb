mod function;
mod expression;
mod application;
mod block;
mod condition;
mod if_block;
mod call;

pub use function::FunctionType;
pub use expression::ExpressionType;
pub use application::ApplicationType;
pub use block::BlockType;
pub use condition::ConditionType;
pub use if_block::IfBlockType;
pub use call::CallType;

pub trait BuilderGenerate {
    type Output;
    fn convert(self) -> Self::Output;
}
