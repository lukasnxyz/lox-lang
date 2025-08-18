use crate::{
    lox::LoxError,
    token::{Object, Token, TokenType},
};
use std::collections::HashMap;

trait CharCheck {
    fn is_lalpha(&self) -> bool;
    fn is_lalphanumeric(&self) -> bool;
}

impl CharCheck for char {
    fn is_lalpha(&self) -> bool {
        (*self >= 'a' && *self <= 'z') || (*self >= 'A' && *self <= 'Z') || *self == '_'
    }

    fn is_lalphanumeric(&self) -> bool {
        self.is_lalpha() || self.is_numeric()
    }
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let keywords = HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]);

        Lexer {
            source: source.to_owned(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            keywords,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn lex_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance()?;
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let amatch = self.amatch('=')?;
                self.add_token(if amatch {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let amatch = self.amatch('=')?;
                self.add_token(if amatch {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let amatch = self.amatch('=')?;
                self.add_token(if amatch {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let amatch = self.amatch('=')?;
                self.add_token(if amatch {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '/' => {
                if self.amatch('/')? {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.advance()?;
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => {
                if c.is_numeric() {
                    self.number()?;
                } else if c.is_lalpha() {
                    self.identifier()?;
                } else {
                    return Err(LoxError::CodeError(
                        self.line,
                        "slice of source (entire line)".to_string(),
                        "encountered an unknown character or sequence of characters".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while self.peek()?.is_lalphanumeric() {
            self.advance()?;
        }

        let text = &self.source[self.start..self.current];
        let token_type = self
            .keywords
            .get(text)
            .unwrap_or(&TokenType::Identifier)
            .clone();

        self.add_token(token_type);

        Ok(())
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while self.peek()?.is_numeric() {
            self.advance()?;
        }

        if self.peek()? == '.' && self.peek_next()?.is_numeric() {
            self.advance()?;

            while self.peek()?.is_numeric() {
                self.advance()?;
            }
        }

        let s = &self.source[self.start..self.current];
        let float_literal = match s.parse::<f64>() {
            Ok(num) => num,
            Err(e) => return Err(LoxError::ParseFloatError(e)),
        };
        self.add_token_literal(TokenType::Number, Object::Number(float_literal));

        Ok(())
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek()? != '"' && !self.is_at_end() {
            if self.peek()? == '\n' {
                self.line += 1;
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(LoxError::CodeError(
                self.line,
                "slice of source (entire line)".to_string(),
                "unterminated string".to_string(),
            ));
        }

        self.advance()?; // closing "

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::LoxString, Object::String(value.to_string()));

        Ok(())
    }

    fn peek(&self) -> Result<char, LoxError> {
        if self.is_at_end() {
            Ok('\0')
        } else {
            match self.source.chars().nth(self.current) {
                Some(c) => Ok(c),
                None => Err(LoxError::EOF),
            }
        }
    }

    fn peek_next(&self) -> Result<char, LoxError> {
        if self.current + 1 >= self.source.len() {
            Ok('\0')
        } else {
            match self.source.chars().nth(self.current + 1) {
                Some(c) => Ok(c),
                None => Err(LoxError::EOF),
            }
        }
    }

    fn amatch(&mut self, expected: char) -> Result<bool, LoxError> {
        if self.is_at_end() {
            return Ok(false);
        }

        match self.source.chars().nth(self.current) {
            Some(curr_char) if curr_char == expected => {
                self.current += 1;
                Ok(true)
            }
            Some(_) => Ok(false),
            None => Err(LoxError::EOF),
        }
    }

    fn advance(&mut self) -> Result<char, LoxError> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        match c {
            Some(c) => Ok(c),
            None => Err(LoxError::EOF),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, Object::None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Object) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start..self.current],
            literal,
            self.line,
        ))
    }

    pub fn lex_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.lex_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", Object::None, self.line));
        Ok(&self.tokens)
    }
}
