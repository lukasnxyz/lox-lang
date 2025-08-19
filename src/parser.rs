use crate::{
    errors::{LoxError, ParseError},
    expression::Expr,
    token::{Object, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    /// expression     → equality ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
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
                value: self.previous().literal, // TODO: Object converts to Expr??
            })
        } else if self.amatch(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "expect ')' after expression")?;
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

            // or err here
            Ok(Expr::Literal {
                value: Object::None,
            })
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, ParseError> {
        if self.check(&token_type) {
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
