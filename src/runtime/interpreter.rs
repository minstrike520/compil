use crate::frontend::ast::{Expression, Program, Statement};

use super::{environment::Environment, values::RuntimeValue};

pub fn evaluate_program(program: Program, environment: &mut Environment) -> RuntimeValue {
    let mut last_evaluated: RuntimeValue = RuntimeValue::NullValue;
    for statement in program.body {
        last_evaluated = evaluate(statement, environment);
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
        _ => panic!("Unexpected operator: {}", operator),
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
fn evaluate_binary_operation(
    left: Box<Expression>,
    right: Box<Expression>,
    operator: String,
    environment: &Environment,
) -> RuntimeValue {
    let left = evaluate_expression(*left, environment);
    let right = evaluate_expression(*right, environment);
    match (left, right) {
        (
            RuntimeValue::NumberValue(left_number_value),
            RuntimeValue::NumberValue(right_number_value),
        ) => RuntimeValue::NumberValue(evaluate_numeric_binary_operation(
            left_number_value,
            right_number_value,
            operator,
        )),
        (_, _) => todo!("Not implemented: evaluating other types of binary operations"),
    }
}

pub fn evaluate_identifier(identifier: String, environment: &Environment) -> RuntimeValue {
    let variable = match environment.lookup_variable(&identifier) {
        Ok(variable) => *variable,
        Err(_) => todo!("VarNotFound handling is not yet implemented"),
    };
    println!("Warning: the implemention of 'evaluate_identifier' is not complete. Unexpected behaviors may occur.");
    variable
}

pub fn evaluate_expression(expression: Expression, environment: &Environment) -> RuntimeValue {
    match expression {
        Expression::NumericLiteral(number) => RuntimeValue::NumberValue(number),
        Expression::Identifier(identifier) => evaluate_identifier(identifier, environment),
        Expression::BinaryExpression {
            left,
            right,
            operator,
        } => evaluate_binary_operation(left, right, operator, environment),
    }
}

pub fn evaluate_variable_declaration(
    identifier: String,
    value: Option<Expression>,
    environment: &mut Environment,
) -> RuntimeValue {
    let value = evaluate_expression(
        value.unwrap_or(Expression::Identifier("null".into())),
        environment,
    );
    match environment.declare_variable(identifier.as_str(), value) {
        Ok(_) => *environment.lookup_variable(&identifier).unwrap(),
        Err(err) => panic!("{:#?}", err),
    }
}

pub fn evaluate_constant_declaration(
    identifier: String,
    value: Expression,
    environment: &mut Environment,
) -> RuntimeValue {
    let value = evaluate_expression(value, environment);
    match environment.declare_constant(identifier.as_str(), value) {
        Ok(_) => *environment.lookup_variable(&identifier).unwrap(),
        Err(err) => panic!("{:#?}", err),
    }
}

pub fn evaluate_assignment(
    assigne: Expression,
    value: Expression,
    environment: &mut Environment,
) -> RuntimeValue {
    if let Expression::Identifier(variable_name) = assigne {
        let value = evaluate_expression(value, environment);
        if let Err(err) = environment.assign_variable(&variable_name, value) {
            panic!("{:#?}", err)
        }
        *environment.lookup_variable(&variable_name).unwrap()
    } else {
        panic!("syntax error: assignment invalid identifier")
    }
}

pub fn evaluate(ast_node: Statement, environment: &mut Environment) -> RuntimeValue {
    match ast_node {
        Statement::Expression(expression) => evaluate_expression(expression, environment),
        Statement::Program(program) => evaluate_program(program, environment),
        Statement::VarDeclaration { identifier, value } => {
            evaluate_variable_declaration(identifier, value, environment)
        }
        Statement::ConstDeclaration { identifier, value } => {
            evaluate_constant_declaration(identifier, value, environment)
        }
        Statement::VarAssignment { assigne, value } => todo!("evaluating variable assignment"),
    }
}
