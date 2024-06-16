use std::{collections::HashMap};

use super::values::RuntimeValue;

#[derive(Debug, Clone)]
pub enum EnvError {
    VarRedefining(String),
    VarNotFound(String),
}

// VarRedefining: format!("Cannot declare variable {variable_name} as it is already defined.")
//
// VarNotFound: format!("Caannot resolve {variable_name} as it does not exist.")

pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeValue>
}

type EnvResult<T> = Result<T, EnvError>;

impl Environment {
    pub fn create(parent: Option<Box<Self>>) -> Self {
        Self{ parent, variables: HashMap::new() }
    }
    pub fn declare_variable(mut self, variable_name: &str, value: RuntimeValue) -> EnvResult<Self> {
        if self.variables.contains_key(variable_name) {
            return Err(EnvError::VarRedefining(variable_name.to_string()));
        }
        self.variables.insert(variable_name.to_string(), value);
        Ok(self)
    }
    pub fn assign_variable(mut self, variable_name: &String, value: RuntimeValue) -> EnvResult<Self> {
        let environment = self.resolve_mut(variable_name)?;
        environment.variables.insert(variable_name.to_string(), value);
        Ok(self)
    }
    pub fn lookup_variable(&self, variable_name: &String) -> EnvResult<&RuntimeValue> {
        let environment = self.resolve(variable_name)?;
        Ok(environment.variables.get(variable_name).unwrap())
    }
    pub fn resolve(&self, variable_name: &String) -> EnvResult<&Self> {
        if self.variables.contains_key(variable_name) { return Ok(self) }
        if let None = self.parent {
            return Err(EnvError::VarNotFound(variable_name.to_string()));
        }
        Ok(self.parent.as_ref().unwrap().resolve(variable_name).unwrap())
    }
    pub fn resolve_mut(&mut self, variable_name: &String) -> EnvResult<&mut Self> {
        if self.variables.contains_key(variable_name) { return Ok(self) }
        if let None = self.parent {
            return Err(EnvError::VarNotFound(variable_name.to_string()));
        }
        Ok(self.parent.as_mut().unwrap().resolve_mut(variable_name).unwrap())
    }
}
