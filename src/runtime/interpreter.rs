use crate::frontend::ast::{BinaryExpression, Expression, NullLiteral, NumericLiteral, Program, Statement};

use super::values::{NullValue, NumberValue, RuntimeValue};


pub fn evaluate_program(program: Program) -> RuntimeValue {
    let mut last_evaluated: RuntimeValue = RuntimeValue::NullValue(NullValue);
    for statement in program.body {
        last_evaluated = evaluate(statement);
    }
    last_evaluated
}

fn evaluate_numeric_binary_expression(left: NumberValue, right: NumberValue, operator: String) -> NumberValue {
    let result = match operator.as_str() {
        "+" => left.value + right.value,
        "-" => left.value - right.value,
        "*" => left.value * right.value,
        "/" => left.value / right.value,
        "%" => left.value % right.value,
        _ => panic!("Unexpected operator: {}", operator)
    };
    NumberValue::create(result)
}
/// # Note: The use of `Box<T>`
/// In this function, I used dereference operator `*` to get the **owned** original value.
/// After this, the box will be just empty.
/// 
/// EXAMPLE
/// ```
/// let a = OwnershipGetter::get(*boxed_value);
/// println!("{}", a.owned); // owned
/// println!("{}", boxed_value) // <ERROR> use of partially moved value: `boxed_value`
/// ```
///
fn evaluate_binary_expression(binary_expression: BinaryExpression) -> RuntimeValue {
    let left = evaluate(Statement::Expression(*binary_expression.left));
    let right = evaluate(Statement::Expression(*binary_expression.right));
    match (left, right) {
        (RuntimeValue::NumberValue(left_number_value), RuntimeValue::NumberValue(right_number_value)) => {
            RuntimeValue::NumberValue(evaluate_numeric_binary_expression(left_number_value, right_number_value, binary_expression.operator))
        },
        (_,_) => panic!()
    }
}

pub fn evaluate(ast_node: Statement) -> RuntimeValue {
    match ast_node {
        Statement::Expression(Expression::NumericLiteral(NumericLiteral{ value })) => {
            RuntimeValue::NumberValue(NumberValue::create(value))
        },
        Statement::Expression(Expression::NullLiteral(NullLiteral)) => {
            RuntimeValue::NullValue(NullValue)
        },
        Statement::Expression(Expression::BinaryExpression(binary_expression)) => {
        evaluate_binary_expression(binary_expression)
    },
        Statement::Program(program) => {
            evaluate_program(program)
        }
        _ => panic!("Unexpected abstract syntax tree node: {:#?}", ast_node)
    }
}
        
