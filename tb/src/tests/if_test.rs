use tb_builder::ApplicationType;
use tb_builder::BlockType;
use tb_builder::ConditionType;
use tb_builder::ExpressionType;
use tb_builder::FunctionType;
use tb_builder::IfBlockType;
use tb_core::types::Number;
use tb_core::types::Value;

use super::get_exit_code;

#[test]
fn equal_1() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::eq(Value::Number(Number::U64(10)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "equal_1", 1);
}

#[test]
fn equal_2() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::eq(Value::Number(Number::U64(9)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "equal_2", 0);
}

#[test]
fn not_equal_1() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ne(Value::Number(Number::U64(10)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "not_equal_1", 0);
}

#[test]
fn not_equal_2() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ne(Value::Number(Number::U64(9)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "not_equal_2", 1);
}

#[test]
fn greater_1() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::gr(Value::Number(Number::U64(11)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "greater_1", 1);
}

#[test]
fn greater_2() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::gr(Value::Number(Number::U64(10)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "greater_2", 0);
}

#[test]
fn greater_3() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::gr(Value::Number(Number::U64(9)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "greater_3", 0);
}

#[test]
fn greater_equal_1() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ge(Value::Number(Number::U64(11)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "greater_equal_1", 1);
}

#[test]
fn greater_equal_2() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ge(Value::Number(Number::U64(10)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "greater_equal_2", 1);
}

#[test]
fn greater_equal_3() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ge    (Value::Number(Number::U64(9)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "greater_equal_3", 0);
}

#[test]
fn less_1() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ls(Value::Number(Number::U64(9)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "less_1", 1);
}

#[test]
fn less_2() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ls(Value::Number(Number::U64(10)), Value::Number(Number::U64(9))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "less_2", 0);
}

#[test]
fn less_3() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::ls(Value::Number(Number::U64(10)), Value::Number(Number::U64(9))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "less_3", 0);
}

#[test]
fn less_equal_1() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::le(Value::Number(Number::U64(10)), Value::Number(Number::U64(11))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "less_equal_1", 1);
}

#[test]
fn less_equal_2() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::le(Value::Number(Number::U64(10)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "less_equal_2", 1);
}

#[test]
fn less_equal_3() {
    let mut main_func = FunctionType::main();
    let mut main_func_block = BlockType::default();

    let mut if_condition_true_block = BlockType::default();
    if_condition_true_block.add_assign("test1", ExpressionType::value(Value::Number(Number::U64(1))));

    let mut if_condition_false_block = BlockType::default();
    if_condition_false_block.add_assign("test1", ExpressionType::value(Value::Number(0.into())));

    let mut if_condition = IfBlockType::default();
    if_condition.set_condition(ConditionType::le(Value::Number(Number::U64(11)), Value::Number(Number::U64(10))));
    if_condition.set_true_block(if_condition_true_block);
    if_condition.set_false_block(if_condition_false_block);

    main_func_block.add_if(if_condition);
    main_func_block.add_return_variable("test1");
    main_func.set_body(main_func_block);

    let mut application_type = ApplicationType::default();
    application_type.add_function(main_func);

    get_exit_code(application_type, "less_equal_3", 0);
}

