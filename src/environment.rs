use crate::{
    errors::EnvError,
    types::{Object, Token},
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Object, EnvError> {
        if let Some(val) = self.values.get(&name.lexeme) {
            return Ok(val.clone());
        }

        match self.enclosing.as_ref() {
            Some(enclosing) => enclosing.get(name),
            None => Err(EnvError::ValueNotFound(
                name.line,
                name.lexeme.clone(),
                format!("no value found for var {}", name.lexeme.clone()),
            )),
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), EnvError> {
        match self.values.get(&name.lexeme) {
            Some(_) => {
                // TODO: do I need an unwrap here?
                self.values.insert(name.lexeme.to_string(), value.clone());
                Ok(())
            }
            None if self.enclosing.is_some() => {
                self.enclosing.as_mut().unwrap().assign(name, value)
            }
            None => Err(EnvError::ValueNotFound(
                name.line,
                name.lexeme.to_string(),
                "undefined variable".to_string(),
            )),
        }
    }
}
