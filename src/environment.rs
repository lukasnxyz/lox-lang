use crate::{
    errors::EnvError,
    token::{Object, Token},
};
use std::collections::HashMap;

pub struct Env {
    values: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Object, EnvError> {
        self.values.get(&name.lexeme).cloned().ok_or_else(|| {
            EnvError::ValueNotFound(
                name.line,
                name.lexeme.clone(),
                format!("no value found for var {}", name.lexeme.clone()),
            )
        })
    }
}
