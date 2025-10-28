use crate::frontend::ast::{Expression, Program, Statement};
use std::collections::HashMap;

use super::values::RuntimeValue;

#[derive(Debug, Clone)]
pub enum EnvError {
    VarRedefining(String),
    VarNotFound(String),
}

// VarRedefining: format!("Cannot declare variable {variable_name} as it is already defined.")
//
// VarNotFound: format!("Caannot resolve {variable_name} as it does not exist.")

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeValue>,
    constants: HashMap<String, RuntimeValue>,
}

type EnvResult<T> = Result<T, EnvError>;

impl Environment {
    pub fn create(parent: Option<Box<Self>>) -> Self {
        Self {
            parent,
            variables: HashMap::new(),
            constants: HashMap::new(),
        }
    }
    pub fn declare_constant(
        &mut self,
        constant_name: &str,
        value: RuntimeValue,
    ) -> EnvResult<&mut Self> {
        if self.constants.contains_key(constant_name) || self.variables.contains_key(constant_name)
        {
            return Err(EnvError::VarRedefining(constant_name.to_string()));
        }
        self.constants.insert(constant_name.to_string(), value);
        Ok(self)
    }
    pub fn declare_variable(
        &mut self,
        variable_name: &str,
        value: RuntimeValue,
    ) -> EnvResult<&mut Self> {
        if self.variables.contains_key(variable_name) || self.constants.contains_key(variable_name)
        {
            return Err(EnvError::VarRedefining(variable_name.to_string()));
        }
        self.variables.insert(variable_name.to_string(), value);
        Ok(self)
    }
    pub fn assign_variable(
        &mut self,
        variable_name: &String,
        value: RuntimeValue,
    ) -> EnvResult<&mut Self> {
        let environment = self.resolve_mut(variable_name)?;
        environment
            .variables
            .insert(variable_name.to_string(), value);
        Ok(self)
    }

    pub fn resolve(&self, variable_name: &String) -> Option<&Self> {
        if self.variables.contains_key(variable_name) || self.constants.contains_key(variable_name)
        {
            return Some(self);
        }
        if let None = self.parent {
            return None;
        }
        Some(self.parent.as_ref().unwrap().resolve(variable_name)?)
    }
    /// TODO
    /// Resolving mechanics don't get constant var.
    pub fn resolve_mut(&mut self, variable_name: &String) -> EnvResult<&mut Self> {
        if self.variables.contains_key(variable_name) {
            return Ok(self);
        }
        if let None = self.parent {
            return Err(EnvError::VarNotFound(variable_name.to_string()));
        }
        Ok(self.parent.as_mut().unwrap().resolve_mut(variable_name)?)
    }
    pub fn evaluate(&mut self, ast_node: Statement) -> EnvResult<&mut Self> {
        match ast_node {
            Statement::Expression(expression) => {
                self.evaluate_expression(expression);
                Ok(self)
            }
            Statement::Program(program) => self.evaluate_program(program),
            Statement::VarDeclaration { identifier, value } => {
                self.evaluate_variable_declaration(identifier.clone(), value)?;
                self.evaluate_identifier(identifier).map(|_| self)
            }
            Statement::ConstDeclaration { identifier, value } => {
                self.evaluate_constant_declaration(identifier.clone(), value)?;
                self.evaluate_identifier(identifier).map(|_| self)
            }
            Statement::VarAssignment { identifier, value } => {
                self.evaluate_variable_assignment(identifier, value)
            }
        }
    }
    pub fn evaluate_program(&mut self, program: Program) -> EnvResult<&mut Self> {
        for statement in program.body {
            self.evaluate(statement)?;
        }
        Ok(self)
    }
    pub fn evaluate_expression(&mut self, expression: Expression) -> EnvResult<RuntimeValue> {
        match expression {
            Expression::NumericLiteral(number) => Ok(RuntimeValue::NumberValue(number)),
            Expression::Identifier(identifier) => self.evaluate_identifier(identifier),
            Expression::BinaryExpression {
                left,
                right,
                operator,
            } => self.evaluate_binary_operation(left, right, operator),
        }
    }
    pub fn evaluate_identifier(&mut self, identifier: String) -> EnvResult<RuntimeValue> {
        if let Some(environment) = self.resolve(&identifier) {
            return Ok(environment.variables.get(&identifier).unwrap().clone());
        } else if let Some(environment) = self.resolve(&identifier) {
            return Ok(environment.constants.get(&identifier).unwrap().clone());
        } else {
            return Err(EnvError::VarNotFound(identifier));
        }
        // TODO other preserved words
    }

    fn evaluate_binary_operation(
        &mut self,
        left: Box<Expression>,
        right: Box<Expression>,
        operator: String,
    ) -> EnvResult<RuntimeValue> {
        let left = self.evaluate_expression(*left)?;
        let right = self.evaluate_expression(*right)?;
        match (left, right) {
            (
                RuntimeValue::NumberValue(left_number_value),
                RuntimeValue::NumberValue(right_number_value),
            ) => Ok(RuntimeValue::NumberValue(
                Environment::evaluate_numeric_binary_operation(
                    left_number_value,
                    right_number_value,
                    operator,
                ),
            )),
            (_, _) => todo!("Not implemented: evaluating other types of binary operations"),
        }
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
    pub fn evaluate_variable_declaration(
        &mut self,
        identifier: String,
        value: Option<Expression>,
    ) -> EnvResult<&mut Self> {
        let value = match value {
            Some(value) => self.evaluate_expression(value)?,
            None => RuntimeValue::NullValue
        };
        self.declare_variable(identifier.as_str(), value)
    }
    pub fn evaluate_constant_declaration(
        &mut self,
        identifier: String,
        value: Expression,
    ) -> EnvResult<&mut Self> {
        let value = self.evaluate_expression(value)?;
        self.declare_constant(identifier.as_str(), value)
    }
    pub fn evaluate_variable_assignment(
        &mut self,
        identifier: String,
        value: Expression,
    ) -> EnvResult<&mut Self> {
        let value = self.evaluate_expression(value);
        let var_ref = match self.variables.get_mut(&identifier) {
            Some(x) => x,
            None => return Err(EnvError::VarNotFound(identifier)),
        };
        *var_ref = value?;
        Ok(self)
    }
}
