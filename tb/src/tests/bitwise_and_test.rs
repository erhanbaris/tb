use tb_builder::ApplicationType;
use tb_builder::BlockType;
use tb_builder::ExpressionType;
use tb_builder::FunctionType;
use tb_core::types::Value;

use super::get_exit_code;

#[test]
fn basic_bitwise_and_test_1() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    main_func_block.add_assign("actual", ExpressionType::bitwise_and(Value::Number(101), Value::Number(1)));
    main_func_block.add_return_variable("actual");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "bitwise_and_test1", 1);
}

#[test]
fn basic_bitwise_and_test_2() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    main_func_block.add_assign("test1", ExpressionType::value(Value::Number(3)));
    main_func_block.add_assign("test2", ExpressionType::value(Value::Number(10)));
    main_func_block.add_assign("actual", ExpressionType::bitwise_and(Value::Variable("test1".to_owned()), Value::Variable("test2".to_owned())));
    main_func_block.add_return_variable("actual");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "bitwise_and_test2", 2);
}