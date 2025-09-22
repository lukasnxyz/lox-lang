use crate::{
  errors::LoxError,
  interpreter::Interpreter,
  types::{Expr, ExprVisitor, Object, Stmt, StmtVisitor, Token},
};

use std::collections::HashMap;

#[derive(Clone)]
enum FunctionType {
  None,
  Function,
}

pub struct Resolver<'a> {
  interpreter: &'a mut Interpreter,
  scopes: Vec<HashMap<String, bool>>, // this is a stack so only push and pop
  current_function: FunctionType,
}

impl<'a> Resolver<'a> {
  pub fn new(interpreter: &'a mut Interpreter) -> Self {
    Self {
      interpreter,
      scopes: vec![],
      current_function: FunctionType::None,
    }
  }

  pub fn resolve_stmts(&mut self, statements: &[Stmt]) {
    statements.iter().for_each(|s| self.resolve_stmt(s));
  }

  fn resolve_stmt(&mut self, statement: &Stmt) {
    statement.accept(self);
  }

  fn resolve_expr(&mut self, expression: &Expr) {
    expression.accept(self);
  }

  fn resolve_local(&mut self, name: &Token) {
    for i in self.scopes.len() - 1..0 {
      if self.scopes[i].contains_key(&name.lexeme) {
        self.interpreter.resolve(
          &Expr::Variable { name: name.clone() },
          self.scopes.len() - 1 - i,
        );
        return;
      }
    }
  }

  fn resolve_function(&mut self, params: &Vec<Token>, body: &Vec<Stmt>, func_type: FunctionType) {
    let enclosing_function = self.current_function.clone();
    self.current_function = func_type;

    self.begin_scope();

    params.iter().for_each(|p| {
      self.declare(p);
      self.define(p);
    });

    self.resolve_stmts(body);
    self.end_scope();

    self.current_function = enclosing_function.clone();
  }

  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new())
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }

  fn declare(&mut self, name: &Token) {
    if let Some(mut scope) = self.scopes.peek_mut() {
      if scope.contains_key(&name.lexeme) {
        LoxError::report(&LoxError::Error);
      }
      scope.try_insert(name.lexeme.clone(), false).unwrap();
    }
  }

  fn define(&mut self, name: &Token) {
    if let Some(mut scope) = self.scopes.peek_mut() {
      if scope.contains_key(&name.lexeme) {
        LoxError::report(&LoxError::SemanticPassError(
          name.line,
          name.lexeme.to_string(),
          "Already a variable with this name in this scope.".to_string(),
        ));
      }
      scope.try_insert(name.lexeme.clone(), false).unwrap();
      scope.try_insert(name.lexeme.clone(), true).unwrap();
    }
  }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
  fn visit_binary_expr(&mut self, left: &Expr, _operator: &Token, right: &Expr) {
    self.resolve_expr(left);
    self.resolve_expr(right);
  }

  fn visit_grouping_expr(&mut self, expression: &Expr) {
    self.resolve_expr(expression);
  }

  fn visit_literal_expr(&mut self, _value: &Object) {
    /* literally nothing because it doesn't mention any variables */
  }

  fn visit_unary_expr(&mut self, _operator: &Token, right: &Expr) {
    self.resolve_expr(right);
  }

  fn visit_var_expr(&mut self, name: &Token) {
    if let Some(last) = self.scopes.last()
      && !last.get(&name.lexeme).unwrap()
    {
      LoxError::report(&LoxError::Error); // TODO: compile time error
    }

    self.resolve_local(name);
  }

  fn visit_assign_expr(&mut self, name: &Token, value: &Expr) {
    self.resolve_expr(value);
    self.resolve_local(name);
  }

  fn visit_logical_expr(&mut self, left: &Expr, _operator: &Token, right: &Expr) {
    self.resolve_expr(left);
    self.resolve_expr(right);
  }

  fn visit_call_expr(&mut self, callee: &Expr, arguments: &[Expr]) {
    self.resolve_expr(callee);
    arguments.iter().for_each(|arg| self.resolve_expr(arg));
  }
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
  fn visit_expression_stmt(&mut self, expression: &Expr) {
    self.resolve_expr(expression);
  }

  fn visit_print_stmt(&mut self, expression: &Expr) {
    self.resolve_expr(expression);
  }

  fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) {
    self.declare(name);
    if let Some(i) = initializer {
      self.resolve_expr(i);
    }
    self.define(name);
  }

  fn visit_block_stmt(&mut self, statements: &[Stmt]) {
    self.begin_scope();
    self.resolve_stmts(statements);
    self.end_scope();
  }

  fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Stmt>) {
    self.resolve_expr(condition);
    self.resolve_stmt(then_branch);
    if let Some(e) = else_branch {
      self.resolve_stmt(e);
    }
  }

  fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) {
    self.resolve_expr(condition);
    self.resolve_stmt(body);
  }

  fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) {
    self.declare(name);
    self.define(name);

    self.resolve_function(params, body, FunctionType::Function);
  }

  fn visit_return_stmt(&mut self, keyword: &Token, value: &Option<Expr>) {
    match self.current_function {
      FunctionType::None => LoxError::report(&LoxError::SemanticPassError(
        keyword.line,
        keyword.lexeme.to_string(),
        "Can't return from top-level code.".to_string(),
      )),
      _ => {}
    }

    if let Some(v) = value {
      self.resolve_expr(v);
    }
  }
}
