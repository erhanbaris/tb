use tb_builder::ApplicationType;
use tb_builder::BlockType;
use tb_builder::ExpressionType;
use tb_builder::FunctionType;
use tb_core::types::Number;
use tb_core::types::NumberType;
use tb_core::types::Value;

use super::check_output;

#[test]
fn func_call_1() {
    let mut sum_func = FunctionType::main();
    sum_func.add_parameter("a", NumberType::I64);
    sum_func.add_parameter("b", NumberType::I64);

    let mut sum_func_block = BlockType::default();
    sum_func_block.add_assign("actual", ExpressionType::add(Value::Variable("a".to_string()), Value::Variable("b".to_string())));
    sum_func_block.add_return_variable("actual");

    sum_func.set_body(sum_func_block);
    sum_func.set_name("sum");

    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    
    main_func_block.add_call_and_assign("sum".to_owned(), vec![20.into(), 12.into()], "total".to_string()); // this is not working now
    main_func_block.add_print("Total value: %d".to_owned(), Some(Value::Variable("total".to_string())));

    main_func_block.add_return_variable("total");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);
    application_type.add_function(sum_func);

    check_output(application_type, "func_call_1", "Total value: 32");
}
