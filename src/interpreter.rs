use crate::{
  callable::{ClockFn, LoxFunction},
  environment::Env,
  errors::{LoxError, RuntimeError},
  types::{Expr, ExprVisitor, Object, Stmt, StmtVisitor, Token, TokenType},
};

use std::{cell::RefCell, rc::Rc};

pub struct Interpreter {
  pub globals: Rc<RefCell<Env>>,
  env: Rc<RefCell<Env>>,
}

impl Interpreter {
  pub fn new() -> Self {
    let globals = Rc::new(RefCell::new(Env::new()));

    globals
      .borrow_mut()
      .define("clock", &Object::Callable(Rc::new(ClockFn)));

    Self {
      globals: globals.clone(),
      env: globals,
    }
  }

  // TODO: an expression alone in a lox file should cause an error or at least a warning
  pub fn interpret(&mut self, statements: Vec<Stmt>, repl: bool) {
    for stmt in statements {
      match stmt.accept(self) {
        Ok(_) if repl => match stmt {
          Stmt::Expression { expression } => match expression.accept(self) {
            Ok(val) => println!("{}", val),
            Err(e) => LoxError::report(&LoxError::RuntimeError(e)),
          },
          _ => {}
        },
        Ok(_) => {}
        Err(e) => LoxError::report(&LoxError::RuntimeError(e)),
      }
    }
  }

  fn check_num_operand(operand: &Object, operator: &Token) -> Result<(), RuntimeError> {
    match operand {
      Object::Number(_) => Ok(()),
      _ => Err(RuntimeError::InvalidType(
        operator.line,
        operator.lexeme.clone(),
        "operand must be a number".to_string(),
      )),
    }
  }

  fn check_num_operands(
    left: &Object,
    right: &Object,
    operator: &Token,
  ) -> Result<(), RuntimeError> {
    match (left, right) {
      (Object::Number(_), Object::Number(_)) => Ok(()),
      _ => Err(RuntimeError::InvalidType(
        operator.line,
        operator.lexeme.clone(),
        "operand must be a number".to_string(),
      )),
    }
  }

  pub fn execute_block(
    &mut self,
    statements: &[Stmt],
    env: Rc<RefCell<Env>>,
  ) -> Result<(), RuntimeError> {
    let previous = self.env.clone();
    self.env = env.clone();
    let result = (|| {
      for stmt in statements {
        stmt.accept(self)?;
      }
      Ok(())
    })();
    self.env = previous;

    result
  }
}

impl ExprVisitor<Result<Object, RuntimeError>> for Interpreter {
  fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, RuntimeError> {
    Ok(value.clone())
  }

  fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, RuntimeError> {
    expression.accept(self)
  }

  fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, RuntimeError> {
    let right = right.accept(self)?;
    match operator.token_type {
      TokenType::Minus => match Self::check_num_operand(&right, operator) {
        Ok(_) => Ok(Object::Number(-right.to_num().unwrap())),
        Err(e) => Err(e),
      },
      TokenType::Bang => Ok(Object::Bool(!right.to_bool())),
      _ => Ok(Object::None),
    }
  }

  fn visit_binary_expr(
    &mut self,
    left: &Expr,
    operator: &Token,
    right: &Expr,
  ) -> Result<Object, RuntimeError> {
    let left = left.accept(self)?;
    let right = right.accept(self)?;

    match operator.token_type {
      TokenType::Greater => Ok(Object::Bool(left > right)),
      TokenType::GreaterEqual => Ok(Object::Bool(left >= right)),
      TokenType::Less => Ok(Object::Bool(left < right)),
      TokenType::LessEqual => Ok(Object::Bool(left <= right)),

      TokenType::BangEqual => Ok(Object::Bool(left != right)),
      TokenType::EqualEqual => Ok(Object::Bool(left == right)),

      TokenType::Minus => {
        Self::check_num_operands(&left, &right, operator)?;
        Ok(Object::Number(
          left.to_num().unwrap() - right.to_num().unwrap(),
        ))
      }
      TokenType::Plus => {
        if left.is_str() && right.is_str() {
          Ok(Object::String(
            left.to_str().unwrap() + &right.to_str().unwrap(),
          ))
        } else if left.is_num() && right.is_num() {
          Ok(Object::Number(
            left.to_num().unwrap() + right.to_num().unwrap(),
          ))
        } else {
          Err(RuntimeError::NumberStringAddition(
            0,
            "".to_string(),
            "can only add variables of the same type".to_string(),
          ))
        }
      }

      TokenType::Slash => {
        Self::check_num_operands(&left, &right, operator)?;
        Ok(Object::Number(
          left.to_num().unwrap() / right.to_num().unwrap(),
        ))
      }
      TokenType::Star => {
        Self::check_num_operands(&left, &right, operator)?;
        Ok(Object::Number(
          left.to_num().unwrap() * right.to_num().unwrap(),
        ))
      }

      _ => Ok(Object::None),
    }
  }

  fn visit_var_expr(&mut self, name: &Token) -> Result<Object, RuntimeError> {
    match self.env.borrow().get(name) {
      Ok(val) => match val {
        Object::None => Err(RuntimeError::VariableUninitialized(
          name.line,
          name.lexeme.clone(),
          "variable uninitialized".to_string(),
        )),
        _ => Ok(val),
      },
      Err(e) => Err(RuntimeError::ValueNotFound(
        name.line,
        name.lexeme.clone(),
        e.to_string(),
      )),
    }
  }

  fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, RuntimeError> {
    let value = value.accept(self)?;
    match self.env.borrow_mut().assign(&name, &value) {
      Ok(_) => Ok(value),
      Err(_) => Err(RuntimeError::ValueNotFound(
        name.line,
        name.lexeme.to_string(),
        "undefined variable".to_string(),
      )),
    }
  }

  fn visit_logical_expr(
    &mut self,
    left: &Expr,
    operator: &Token,
    right: &Expr,
  ) -> Result<Object, RuntimeError> {
    let left = left.accept(self)?;

    match operator.token_type {
      TokenType::Or => {
        if left.to_bool() {
          return Ok(left);
        }
      }
      _ => {
        if !left.to_bool() {
          return Ok(left);
        }
      }
    }

    right.accept(self)
  }

  fn visit_call_expr(&mut self, callee: &Expr, arguments: &[Expr]) -> Result<Object, RuntimeError> {
    let callee = callee.accept(self)?;

    let mut ret_arguments = vec![];
    for arg in arguments {
      ret_arguments.push(arg.accept(self)?);
    }

    let function = callee.as_callable()?; // this contains the runtime type check
    if ret_arguments.len() != function.arity() {
      return Err(RuntimeError::InvalidNumArgs(format!(
        "expected {} arguments, but got {}",
        function.arity(),
        ret_arguments.len()
      )));
    }

    Ok(function.call(self, &ret_arguments)?)
  }
}

impl StmtVisitor<Result<(), RuntimeError>> for Interpreter {
  fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
    expression.accept(self)?;
    Ok(())
  }

  fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), RuntimeError> {
    let value = expression.accept(self)?;
    println!("{}", value);
    Ok(())
  }

  fn visit_var_stmt(
    &mut self,
    name: &Token,
    initializer: &Option<Expr>,
  ) -> Result<(), RuntimeError> {
    let mut value = Object::None;
    match initializer {
      Some(val) => value = val.accept(self)?,
      None => {}
    }

    // TODO: good place to put a warning that var is uninited or something

    self.env.borrow_mut().define(&name.lexeme, &value);
    Ok(())
  }

  fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
    let n_env = Rc::new(RefCell::new(Env::new_enclosing(self.env.clone())));
    self.execute_block(statements, n_env)
  }

  fn visit_if_stmt(
    &mut self,
    condition: &Expr,
    then_branch: &Stmt,
    else_branch: &Option<Stmt>,
  ) -> Result<(), RuntimeError> {
    if condition.accept(self)?.to_bool() {
      then_branch.accept(self)?;
    } else if let Some(e_branch) = else_branch {
      e_branch.accept(self)?;
    }

    Ok(())
  }

  fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
    while condition.accept(self)?.to_bool() {
      body.accept(self)?;
    }

    Ok(())
  }

  fn visit_function_stmt(
    &mut self,
    name: &Token,
    params: &Vec<Token>,
    body: &Vec<Stmt>,
  ) -> Result<(), RuntimeError> {
    let function = Object::Callable(Rc::new(LoxFunction::new(
      Stmt::Function {
        name: name.clone(),
        params: params.to_vec(),
        body: body.to_vec(),
      },
      Rc::clone(&self.env),
    )));
    self.env.borrow_mut().define(&name.lexeme, &function);

    Ok(())
  }

  fn visit_return_stmt(
    &mut self,
    _keyword: &Token,
    value: &Option<Expr>,
  ) -> Result<(), RuntimeError> {
    let mut ret_value: Option<Object> = None;
    if let Some(v) = value {
      ret_value = Some(v.accept(self)?);
    }

    // NOTE: this is to be like an exception in Java to unwind the call stack
    Err(RuntimeError::ReturnCalled(ret_value))
  }

  /*
  fn visit_class_stmt(
    &mut self,
    name: &Token,
    superclass: &Expr,
    methods: &Vec<Stmt>,
  ) -> Result<(), RuntimeError> {
    Ok(())
  }
  */
}
