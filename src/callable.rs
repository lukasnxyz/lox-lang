use crate::{
  environment::Env,
  errors::RuntimeError,
  interpreter::Interpreter,
  types::{Object, Stmt, Token},
};
use std::{cell::RefCell, fmt, rc::Rc, time};

pub trait Callable: fmt::Display {
  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: &[Object],
  ) -> Result<Object, RuntimeError>;
  fn arity(&self) -> usize;
}

pub struct ClockFn;

impl fmt::Display for ClockFn {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<fn>")
  }
}

impl Callable for ClockFn {
  fn call(
    &self,
    _interpreter: &mut Interpreter,
    _arguments: &[Object],
  ) -> Result<Object, RuntimeError> {
    let now = time::SystemTime::now()
      .duration_since(time::UNIX_EPOCH)
      .unwrap(); // TODO: no unwrap here

    Ok(Object::Number(now.as_secs_f64()))
  }

  fn arity(&self) -> usize {
    0
  }
}

#[derive(Clone)]
pub struct LoxFunction {
  declaration: Stmt, // Stmt::Function
  closure: Rc<RefCell<Env>>,
}

impl LoxFunction {
  pub fn new(declaration: Stmt, closure: Rc<RefCell<Env>>) -> Self {
    Self {
      declaration,
      closure,
    }
  }
}

impl fmt::Display for LoxFunction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Stmt::Function { name, .. } = &self.declaration {
      write!(f, "<fn> {}", name.lexeme)
    } else {
      write!(f, "function not callabe")
    }
  }
}

impl Callable for LoxFunction {
  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: &[Object],
  ) -> Result<Object, RuntimeError> {
    if let Stmt::Function {
      name: _,
      params,
      body,
    } = &self.declaration
    {
      let mut environment = Env::new_enclosing(Rc::clone(&self.closure));

      for (param, arg) in params.iter().zip(arguments.iter()) {
        environment.define(&param.lexeme, &arg);
      }

      match interpreter.execute_block(&body, Rc::new(RefCell::new(environment))) {
        Ok(_) => {}
        Err(RuntimeError::ReturnCalled(val)) => {
          if let Some(val) = val {
            return Ok(val);
          }
        }
        Err(e) => return Err(e),
      }

      Ok(Object::None)
    } else {
      Err(RuntimeError::InvalidFunctionCall(
        0,
        "no lexeme".to_string(),
        "call is not callable/function does not exist".to_string(),
      ))
    }
  }

  fn arity(&self) -> usize {
    if let Stmt::Function { params, .. } = &self.declaration {
      params.len()
    } else {
      0
    }
  }
}
