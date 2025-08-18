use crate::red_text;
use std::{fmt, io};

#[derive(Debug)]
pub enum LoxError {
    Io(io::Error),
    ParseFloatError(std::num::ParseFloatError),
    Eof,
    Error,
    LexError(LexError),
    ParseError(ParseError),
    RuntimeError(RuntimeError),
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Io(e) => write!(f, "io error: {}", e),
            LoxError::ParseFloatError(e) => write!(f, "parse float error: {}", e),
            LoxError::Eof => write!(f, "hit eof in the middle of parsing"),
            LoxError::Error => write!(f, "error"),
            LoxError::LexError(e) => write!(f, "{}", e),
            LoxError::ParseError(e) => write!(f, "{}", e),
            LoxError::RuntimeError(e) => write!(f, "{}", e),
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

impl LoxError {
    pub fn report(err: &LoxError) {
        println!("{}", err);
    }
}

// line, lexeme, msg
#[derive(Debug)]
pub enum LexError {
    IncompleteString(usize, String, String),
    UnknownChar(usize, String, String),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::IncompleteString(line, lexeme, msg) => write!(
                f,
                "{}: {}\n\t[Line {} Error in '{}']: {}",
                red_text!("error"),
                "LexError::IncompleteString",
                line,
                lexeme,
                msg
            ),
            LexError::UnknownChar(line, lexeme, msg) => write!(
                f,
                "{}: {}\n\t[Line {} Error in '{}']: {}",
                red_text!("error"),
                "LexError::UnknownChar",
                line,
                lexeme,
                msg
            ),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidExpression(usize, String, String),
    EndOfExpression(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidExpression(line, lexeme, msg) => write!(
                f,
                "{}: {}\n\t[Line {} Error in '{}']: {}",
                red_text!("error"),
                "ParseError::InvalidExpression",
                line,
                lexeme,
                msg
            ),
            ParseError::EndOfExpression(msg) => {
                write!(
                    f,
                    "{}: {}\n\t[Error]: {}",
                    red_text!("error"),
                    "ParseError::EndOfExpression",
                    msg
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    InvalidTypes(usize, String, String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::InvalidTypes(line, lexeme, msg) => write!(
                f,
                "{}: {}\n\t[Line {} Error in '{}']: {}",
                red_text!("error"),
                "RuntimeError::InvalidTypes",
                line,
                lexeme,
                msg
            ),
        }
    }
}
