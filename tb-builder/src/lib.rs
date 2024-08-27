mod function;
mod expression;
mod application;

pub use function::FunctionType;
pub use expression::ExpressionType;
pub use application::ApplicationType;

pub trait BuilderGenerate {
    type Output;
    fn convert(self) -> Self::Output;
}
