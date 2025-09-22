use crate::{
  errors::EnvError,
  types::{Object, Token},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Default)]
pub struct Env {
  values: HashMap<String, Object>,
  enclosing: Option<Rc<RefCell<Env>>>,
}

impl Env {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
      enclosing: None,
    }
  }

  pub fn new_enclosing(enclosing: Rc<RefCell<Env>>) -> Self {
    Self {
      values: HashMap::new(),
      enclosing: Some(enclosing),
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

  pub fn get_at(env: Rc<RefCell<Self>>, distance: i32, name: &str) -> Option<Object> {
    Self::ancestor(env, distance)
      .borrow()
      .values
      .get(name)
      .cloned()
  }

  pub fn assign_at(env: Rc<RefCell<Self>>, distance: i32, name: &Token, value: &Object) {
    Self::ancestor(env, distance)
      .borrow_mut()
      .values
      .insert(name.lexeme.to_string(), value.clone());
  }

  fn ancestor(env: Rc<RefCell<Self>>, distance: i32) -> Rc<RefCell<Env>> {
    // TODO: this is wrong because we don't want to clone but instead get a dependent copy
    let mut environment = env;
    for _ in 0..distance {
      let next = environment
        .borrow()
        .enclosing
        .clone()
        .expect("No enclosing environment found at this distance");
      environment = next;
    }
    environment
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
}
