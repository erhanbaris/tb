mod function;
mod expression;
mod application;
mod block;

pub use function::FunctionType;
pub use expression::ExpressionType;
pub use application::ApplicationType;
pub use block::BlockType;

pub trait BuilderGenerate {
    type Output;
    fn convert(self) -> Self::Output;
}
