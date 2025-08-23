use crate::{
    errors::EnvError,
    types::{Object, Token},
};
use std::{
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
};

#[derive(Clone, Default)]
pub struct Env {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new(enclosing: Option<Rc<RefCell<Env>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: enclosing,
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, EnvError> {
        if let Some(val) = self.values.get(&name.lexeme) {
            return Ok(val.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().get(name);
        }

        Err(EnvError::ValueNotFound(
                name.line,
                name.lexeme.clone(),
                format!("no value found for var {}", name.lexeme.clone()),
        ))
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), EnvError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }

        if let Some(enclosing) = self.enclosing.as_mut() {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(EnvError::ValueNotFound(
                name.line,
                name.lexeme.to_string(),
                "undefined variable".to_string(),
        ))
    }

    /*
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
    */
}
