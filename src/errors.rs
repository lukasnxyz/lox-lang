use crate::{error_indent, red_text};
use std::{fmt, io};

#[derive(Debug)]
pub enum LoxError {
    Io(io::Error),
    Error,
    LexError(LexError),
    ParseError(ParseError),
    RuntimeError(RuntimeError),
    EnvError(EnvError),
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Io(e) => write!(f, "io error: {}", e),
            LoxError::Error => write!(f, "error"),
            LoxError::LexError(e) => write!(f, "{}", e),
            LoxError::ParseError(e) => write!(f, "{}", e),
            LoxError::RuntimeError(e) => write!(f, "{}", e),
            LoxError::EnvError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for LoxError {}

impl From<io::Error> for LoxError {
    fn from(err: io::Error) -> LoxError {
        LoxError::Io(err)
    }
}

impl LoxError {
    pub fn report(err: &LoxError) {
        println!("{}", err);
    }
}

// line, lexeme, msg
#[derive(Debug, Clone)]
pub enum LexError {
    IncompleteString(usize, String, String),
    UnknownChar(usize, String, String),
    ParseFloatError(std::num::ParseFloatError),
    Eof,
}

impl From<std::num::ParseFloatError> for LexError {
    fn from(err: std::num::ParseFloatError) -> LexError {
        LexError::ParseFloatError(err)
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::IncompleteString(line, lexeme, msg) => write!(
                f,
                "{}: {}\n{}[Line {} Error in '{}']: {}",
                red_text!("error"),
                "LexError::IncompleteString",
                error_indent!(),
                line,
                lexeme,
                msg
            ),
            LexError::UnknownChar(line, lexeme, msg) => write!(
                f,
                "{}: {}\n{}[Line {} Error in '{}']: {}",
                red_text!("error"),
                "LexError::UnknownChar",
                error_indent!(),
                line,
                lexeme,
                msg
            ),
            LexError::ParseFloatError(e) => write!(f, "parse float error: {}", e),
            LexError::Eof => write!(f, "hit eof while lexing"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidExpression(usize, String, String),
    InvalidAssignment(usize, String, String),
    EndOfExpression(String),
    Error(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidExpression(line, lexeme, msg) => write!(
                f,
                "{}: {}\n{}[Line {} Error in '{}']: {}",
                red_text!("error"),
                "ParseError::InvalidExpression",
                error_indent!(),
                line,
                lexeme,
                msg
            ),
            ParseError::InvalidAssignment(line, lexeme, msg) => write!(
                f,
                "{}: {}\n{}[Line {} Error in '{}']: {}",
                red_text!("error"),
                "ParseError::InvalidAssignment",
                error_indent!(),
                line,
                lexeme,
                msg
            ),
            ParseError::EndOfExpression(msg) => {
                write!(
                    f,
                    "{}: {}\n{}[Error]: {}",
                    red_text!("error"),
                    "ParseError::EndOfExpression",
                    error_indent!(),
                    msg
                )
            }
            ParseError::Error(msg) => {
                write!(
                    f,
                    "{}: {}\n{}[Error]: {}",
                    red_text!("error"),
                    "ParseError::Error",
                    error_indent!(),
                    msg
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    InvalidType(usize, String, String),
    NumberStringAddition(usize, String, String),
    ValueNotFound(usize, String, String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::InvalidType(line, lexeme, msg) => write!(
                f,
                "{}: {}\n{}[Line {} Error in '{}']: {}",
                red_text!("error"),
                "RuntimeError::InvalidType",
                error_indent!(),
                line,
                lexeme,
                msg
            ),
            RuntimeError::NumberStringAddition(line, lexeme, msg) => write!(
                f,
                "{}: {}\n{}[Line {} Error in '{}']: {}",
                red_text!("error"),
                "RuntimeError::NumberStringAddition",
                error_indent!(),
                line,
                lexeme,
                msg
            ),
            RuntimeError::ValueNotFound(line, lexeme, msg) => write!(
                f,
                "{}: {}\n{}[Line {} Error in '{}']: {}",
                red_text!("error"),
                "RuntimeError::UnknownVariable",
                error_indent!(),
                line,
                lexeme,
                msg
            ),
        }
    }
}

#[derive(Debug)]
pub enum EnvError {
    ValueNotFound(usize, String, String),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnvError::ValueNotFound(line, lexeme, msg) => write!(
                f,
                "{}: {}\n{}[Line {} Error in '{}']: {}",
                red_text!("error"),
                "EnvError::ValueNotFound",
                error_indent!(),
                line,
                lexeme,
                msg
            ),
        }
    }
}
