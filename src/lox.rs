use crate::{
    lexer::Lexer,
    parser::Parser,
    token::{Token, TokenType},
};
use std::{fmt, fs, io, io::Write};

#[derive(Debug)]
pub enum LoxError {
    Io(io::Error),
    ParseFloatError(std::num::ParseFloatError),
    EOF,
    CodeError(usize, String, String), // line, where, msg
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Io(e) => write!(f, "io error: {}", e),
            LoxError::ParseFloatError(e) => write!(f, "parse float error: {}", e),
            LoxError::EOF => write!(f, "hit eof in the middle of parsing"),
            LoxError::CodeError(line, location, msg) => {
                write!(f, "[Line {} Error in {}]: {}", line, location, msg)
            }
        }
    }
}

impl std::error::Error for LoxError {}

impl From<io::Error> for LoxError {
    fn from(err: io::Error) -> LoxError {
        LoxError::Io(err)
    }
}

impl From<std::num::ParseFloatError> for LoxError {
    fn from(err: std::num::ParseFloatError) -> LoxError {
        LoxError::ParseFloatError(err)
    }
}

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    fn run(source: &str) -> Result<(), LoxError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let expression = parser.parse().unwrap();

        println!("expr: {}", expression);

        Ok(())
    }

    pub fn run_file(&self, path: &str) -> Result<(), LoxError> {
        let source = fs::read_to_string(path)?;
        Self::run(&source)?;
        if self.had_error {
            return Err(LoxError::CodeError(0, "".to_string(), "".to_string()));
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), LoxError> {
        loop {
            let mut input = String::new();
            print!(">> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input)?;
            if input.trim().is_empty() {
                continue;
            }
            Self::run(&input)?;
            self.had_error = false;
        }
    }

    pub fn error(token: &Token, msg: &str) {
        if token.token_type == TokenType::Eof {
            Self::report_error(token.line, " at end", msg);
        } else {
            Self::report_error(token.line, &format!(" at '{}'", token.lexeme), msg);
        }
    }

    fn report_error(line: usize, lexeme_where: &str, msg: &str) {
        println!(
            "\x1b[31merror: \x1b[0m {}\n  -->{}: {}",
            msg, line, lexeme_where,
        );
    }
}
