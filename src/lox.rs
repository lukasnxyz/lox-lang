use crate::{
    lexer::Lexer,
    parser::Parser,
    expression::Expr,
};
use std::{io, fmt, fs};

#[derive(Debug)]
pub enum LoxError {
    Io(io::Error),
    ParseFloatError(),
    InvalidPrompt(Strign),
    EOF,
    CodeError(usize, String, String), // line, where, msg
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Io(e) => write!(f, "io error: {}", e),
            LoxError::ParseFloatError(e) => write!(f, "parse float error: {}", e),
            LoxError::InvalidPrompt(msg) => write!(f, "invalid input: {}", msg),
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

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn run(source: &str) -> Result<(), LoxError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        println!("expr: {}", expression);

        let source = fs::read_to_string(path)?;
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<(), LoxError> {
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

    fn error(token: &Token, msg: &str) {
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

