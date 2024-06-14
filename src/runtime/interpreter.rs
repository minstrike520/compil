use crate::frontend::ast::{Expression, Program, Statement};

use super::values::RuntimeValue;


pub fn evaluate_program(program: Program) -> RuntimeValue {
    let mut last_evaluated: RuntimeValue = RuntimeValue::NullValue;
    for statement in program.body {
        last_evaluated = evaluate(statement);
    }
    last_evaluated
}

fn evaluate_numeric_binary_operation(left: i32, right: i32, operator: String) -> i32 {
    let result = match operator.as_str() {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        "%" => left % right,
        _ => panic!("Unexpected operator: {}", operator)
    };
    result
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
fn evaluate_binary_operation(left: Box<Expression>, right: Box<Expression>, operator: String) -> RuntimeValue {
    let left = evaluate_expression(*left);
    let right = evaluate_expression(*right);
    match (left, right) {
        (RuntimeValue::NumberValue(left_number_value), RuntimeValue::NumberValue(right_number_value)) => {
            RuntimeValue::NumberValue(evaluate_numeric_binary_operation(left_number_value, right_number_value, operator))
        },
        (_,_) => todo!("Not implemented: evaluating other types of binary operations")
    }
}

pub fn evaluate_expression(expression: Expression) -> RuntimeValue {
    match expression {
        Expression::NullLiteral => RuntimeValue::NullValue,
        Expression::NumericLiteral(number) => RuntimeValue::NumberValue(number),
        Expression::Identifier(identifier) => todo!("Not implemented: evaluating identifier"),
        Expression::BinaryExpression { left, right, operator } => evaluate_binary_operation(left, right, operator),
    }
}

pub fn evaluate(ast_node: Statement) -> RuntimeValue {
    match ast_node {
        Statement::Expression(expression) => evaluate_expression(expression),
        Statement::Program(program) => {
            evaluate_program(program)
        }
    }
}
        
