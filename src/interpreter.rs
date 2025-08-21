use crate::{
    environment::Env,
    errors::{LoxError, RuntimeError},
    expression::{Expr, ExprVisitor},
    stmt::{Stmt, StmtVisitor},
    token::{Object, Token, TokenType},
};

pub struct Interpreter {
    pub env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            match stmt.accept(self) {
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

    // TODO: are all of these clones correct?
    fn execute_block(&mut self, statements: Vec<Stmt>, env: &Env) -> Result<(), RuntimeError> {
        let previous = self.env.clone();
        self.env = env.clone();
        for stmt in statements {
            stmt.accept(self)?;
        }
        self.env = previous;

        Ok(())
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

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, RuntimeError> {
        match self.env.get(name) {
            Ok(val) => Ok(val),
            Err(e) => Err(RuntimeError::ValueNotFound(
                name.line,
                name.lexeme.clone(),
                e.to_string(),
            )),
        }
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, RuntimeError> {
        let value = value.accept(self)?;
        match self.env.assign(&name, &value) {
            Ok(_) => Ok(value),
            Err(_) => Err(RuntimeError::ValueNotFound(
                name.line,
                name.lexeme.to_string(),
                "undefined variable".to_string(),
            )),
        }
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

        self.env.define(&name.lexeme, &value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        let env = self.env.clone();
        self.execute_block(statements.to_vec(), &env)?;
        Ok(())
    }

    /*
    fn visit_class_stmt(
        &self,
        name: &Token,
        superclass: &Expr,
        methods: &Vec<Stmt>,
    ) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn visit_function_stmt(
        &self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn visit_if_stmt(
        &self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Stmt,
    ) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn visit_return_stmt(&self, keyword: &Token, value: &Expr) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn visit_while_stmt(&self, condition: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
        Ok(())
    }
    */
}
