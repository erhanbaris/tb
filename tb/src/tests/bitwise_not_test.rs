use tb_builder::ApplicationType;
use tb_builder::BlockType;
use tb_builder::ExpressionType;
use tb_builder::FunctionType;
use tb_core::types::Value;

use super::get_exit_code;

#[test]
fn basic_bitwise_not_test_1() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    main_func_block.add_assign("actual", ExpressionType::bitwise_not(Value::Number(4294967292_u64.into())));
    main_func_block.add_return_variable("actual");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "bitwise_not_test1", 3);
}
