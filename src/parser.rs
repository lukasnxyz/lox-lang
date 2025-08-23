use crate::{
    errors::{LoxError, ParseError},
    types::{Expr, Object, Stmt, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];
        while !self.peek().is_eof() {
            match self.declaration() {
                Ok(val) => statements.push(val),
                Err(e) => {
                    self.had_error = true;
                    self.synchronize();
                    LoxError::report(&LoxError::ParseError(e));
                }
            }
        }

        if self.had_error {
            Err(ParseError::Error(
                "one or more parsing errors have occured".to_string(),
            ))
        } else {
            Ok(statements)
        }
    }

    /// expression     → equality ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.amatch(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
        // TODO: should synchronize and return None if error
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.amatch(&[TokenType::For]) {
            self.for_statement()
        } else if self.amatch(&[TokenType::If]) {
            self.if_statement()
        } else if self.amatch(&[TokenType::Print]) {
            self.print_stmt()
        } else if self.amatch(&[TokenType::While]) {
            self.while_statement()
        } else if self.amatch(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block {
                statements: self.block()?,
            })
        } else {
            self.expr_stmt()
        }
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParen, "expect '(' after 'for'")?;

        let mut initializer = None;
        if self.amatch(&[TokenType::Semicolon]) {
            initializer = None;
        } else if self.amatch(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expr_stmt()?);
        }

        let mut condition = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(&TokenType::Semicolon, "expect ';' after loop condition")?;

        let mut increment = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(&TokenType::RightParen, "expect ')' after for clauses")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Stmt::Block {
                statements: vec![body, Stmt::Expression { expression: inc }],
            };
        }

        body = Stmt::While {
            condition: condition.unwrap_or(Expr::Literal {
                value: Object::Bool(true),
            }),
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![init, body],
            };
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParen, "expect '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "expect ')' after condition")?;
        let body = self.statement()?;

        Ok(Stmt::While {
            condition: condition,
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(&TokenType::LeftParen, "expect '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "expect ')' after if condition")?;

        let then_branch = self.statement()?;
        let mut else_branch: Box<Option<Stmt>> = Box::new(None);
        if self.amatch(&[TokenType::Else]) {
            else_branch = Box::new(Some(self.statement()?));
        }

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "expect ';' after value")?;
        Ok(Stmt::Print { expression: value })
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(&TokenType::Identifier, "expect variable name")?;

        let mut initializer = None;
        if self.amatch(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            &TokenType::Semicolon,
            "expect ';' after variable declaration",
        )?;
        Ok(Stmt::Var {
            name: name,
            initializer: initializer,
        })
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "expect ';' after value")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];
        while !self.check(&TokenType::RightBrace) {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "expect '}' after block")?;

        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if !self.amatch(&[TokenType::Equal]) {
            return Ok(expr);
        }

        let equals = self.previous();
        let value = self.assignment()?;

        match expr {
            Expr::Variable { name } => Ok(Expr::Assign {
                name,
                value: Box::new(value),
            }),
            _ => Err(ParseError::InvalidAssignment(
                equals.line,
                equals.lexeme,
                "invalid assignment target".to_string(),
            )),
        }
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.amatch(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.amatch(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.amatch(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn amatch(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.peek().is_eof() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.peek().is_eof() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&self) -> Token {
        // TODO: get rid of unwrap and clone
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        // TODO: get rid of unwrap and clone
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.amatch(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.amatch(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.amatch(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// unary          → ( "!" | "-" ) unary
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.amatch(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    /// primary        → NUMBER | STRING | "true" | "false" | "none" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.amatch(&[TokenType::False]) {
            Ok(Expr::Literal {
                value: Object::Bool(false),
            })
        } else if self.amatch(&[TokenType::True]) {
            Ok(Expr::Literal {
                value: Object::Bool(true),
            })
        } else if self.amatch(&[TokenType::None]) {
            Ok(Expr::Literal {
                value: Object::None,
            })
        } else if self.amatch(&[TokenType::Number, TokenType::LoxString]) {
            Ok(Expr::Literal {
                value: self.previous().literal,
            })
        } else if self.amatch(&[TokenType::Identifier]) {
            Ok(Expr::Variable {
                name: self.previous(),
            })
        } else if self.amatch(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "expect ')' after expression")?;
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else {
            let err = LoxError::ParseError(ParseError::InvalidExpression(
                self.peek().line,
                self.peek().lexeme,
                "expect expression".to_string(),
            ));
            LoxError::report(&err);
            self.synchronize();

            // TODO: possibly remove this all here as its handled in the parse func now

            // or err here
            Ok(Expr::Literal {
                value: Object::None,
            })
        }
    }

    fn consume(&mut self, token_type: &TokenType, msg: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::EndOfExpression(msg.to_string()))
        }
    }

    // pretty easy to jump forwards to the next statement as you just have to jump forward to the
    // next semicolon (in most cases a semicolon will indicate a next statement)
    fn synchronize(&mut self) {
        self.advance();

        while !self.peek().is_eof() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class => {}
                TokenType::Fun => {}
                TokenType::Var => {}
                TokenType::For => {}
                TokenType::If => {}
                TokenType::While => {}
                TokenType::Print => {}
                TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
