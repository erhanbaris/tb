use tb_core::types::{Condition, ConditionDiscriminant, Value};

use crate::{instruction::X86Instruction, X86ApplicationContext, X86Store};

use super::{error::X86Error, X86ValueCompiler};

pub struct X86ConditionCompiler;


impl X86ConditionCompiler {
    pub fn compile(condition: Condition, scope: &mut X86Store, context: &mut X86ApplicationContext) -> Result<ConditionDiscriminant, X86Error> {
        match condition {
            Condition::Eq { left, right } => Self::compile_simple(scope, ConditionDiscriminant::Eq, left, right, context),
            Condition::Ne { left, right } => Self::compile_simple(scope, ConditionDiscriminant::Ne, left, right, context),
            Condition::Gr { left, right } => Self::compile_simple(scope, ConditionDiscriminant::Gr, left, right, context),
            Condition::Ge { left, right } => Self::compile_simple(scope, ConditionDiscriminant::Ge, left, right, context),
            Condition::Ls { left, right } => Self::compile_simple(scope, ConditionDiscriminant::Ls, left, right, context),
            Condition::Le { left, right } => Self::compile_simple(scope, ConditionDiscriminant::Le, left, right, context),
        }
    }
    
    fn compile_simple(scope: &mut X86Store, condition_type: ConditionDiscriminant, left: Value, right: Value, context: &mut X86ApplicationContext) -> Result<ConditionDiscriminant, X86Error> {
        let registers = scope.register_backup();

        context.instructions.add_comment("Generate right value".to_owned());
        let right = X86ValueCompiler::compile(right, context, scope, None)?;
        
        context.instructions.add_comment("Generate left value".to_owned());
        let left = X86ValueCompiler::compile(left, context, scope, None)?;

        context.instructions.add_instruction(X86Instruction::Cmp { left, right, comment: None });
        scope.register_restore(registers);

        Ok(condition_type)
    }
}
