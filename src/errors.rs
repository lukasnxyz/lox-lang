use crate::{error_indent, red_text, types::Object};
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
  MaxNumFuncParameters(usize, String, String),
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
      ParseError::MaxNumFuncParameters(line, lexeme, msg) => write!(
        f,
        "{}: {}\n{}[Line {} Error in '{}']: {}",
        red_text!("error"),
        "ParseError::MaxNumFuncParameters",
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

pub enum RuntimeError {
  InvalidType(usize, String, String),
  NumberStringAddition(usize, String, String),
  ValueNotFound(usize, String, String),
  VariableUninitialized(usize, String, String),
  InvalidFunctionCall(usize, String, String),
  ReturnCalled(Option<Object>),
  InvalidNumArgs(String),
}

impl fmt::Debug for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      RuntimeError::InvalidType(line, got, expected) => {
        write!(
          f,
          "[line {}] Invalid type: got {}, expected {}",
          line, got, expected
        )
      }
      RuntimeError::NumberStringAddition(line, left, right) => {
        write!(
          f,
          "[line {}] Cannot add number {} and string {}",
          line, left, right
        )
      }
      RuntimeError::ValueNotFound(line, name, msg) => {
        write!(f, "[line {}] Value not found: {} ({})", line, name, msg)
      }
      RuntimeError::VariableUninitialized(line, name, msg) => {
        write!(
          f,
          "[line {}] Variable '{}' uninitialized ({})",
          line, name, msg
        )
      }
      RuntimeError::InvalidFunctionCall(line, name, msg) => {
        write!(
          f,
          "[line {}] Invalid function call '{}' ({})",
          line, name, msg
        )
      }
      RuntimeError::ReturnCalled(val) => match val {
        Some(obj) => write!(f, "Return called with value: {}", obj),
        None => write!(f, "Return called with no value"),
      },
      RuntimeError::InvalidNumArgs(msg) => {
        write!(f, "Invalid number of arguments: {}", msg)
      }
    }
  }
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
      RuntimeError::VariableUninitialized(line, lexeme, msg) => write!(
        f,
        "{}: {}\n{}[Line {} Error in '{}']: {}",
        red_text!("error"),
        "RuntimeError::VariableUninitialized",
        error_indent!(),
        line,
        lexeme,
        msg
      ),
      RuntimeError::InvalidFunctionCall(line, lexeme, msg) => write!(
        f,
        "{}: {}\n{}[Line {} Error in '{}']: {}",
        red_text!("error"),
        "RuntimeError::InvalidFunctionCall",
        error_indent!(),
        line,
        lexeme,
        msg
      ),
      RuntimeError::ReturnCalled(obj) => write!(
        f,
        "{}: {}\n{}[Error]: {}",
        red_text!("error"),
        "RuntimeError::ReturnCalled",
        error_indent!(),
        if let Some(o) = obj { o } else { &Object::None }
      ),
      RuntimeError::InvalidNumArgs(msg) => write!(
        f,
        "{}: {}\n{}[Error]: {}",
        red_text!("error"),
        "RuntimeError::InvalidNumArgs",
        error_indent!(),
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
