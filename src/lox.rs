use crate::{errors::LoxError, interpreter::Interpreter, lexer::Lexer, parser::Parser};
use std::{
  fs,
  io::{self, Write},
  path::Path,
};

pub struct Lox;

impl Lox {
  pub fn new() -> Self {
    Self {}
  }

  fn run(source: &str, repl: bool) -> Result<(), LoxError> {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.lex_tokens() {
      Ok(tokens) => tokens,
      Err(e) => {
        LoxError::report(&LoxError::LexError(e.clone()));
        return Err(LoxError::LexError(e));
      }
    };

    let mut parser = Parser::new(tokens);
    let statements = match parser.parse() {
      Ok(statements) => statements,
      Err(e) => {
        LoxError::report(&LoxError::ParseError(e.clone()));
        return Err(LoxError::ParseError(e));
      }
    };

    // TODO: can print the statements here but need to implement an AstPrint for it

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements, repl);

    Ok(())
  }

  pub fn run_file(&self, path: &str) -> Result<(), LoxError> {
    let source = fs::read_to_string(Path::new(path))?;
    Self::run(&source, false)?;
    Ok(())
  }

  // TODO: ctrl-c does nothing, ctrl-d quits
  // TODO: up and down arrow for history
  // TODO: left and right arrow for editing text
  pub fn run_prompt(&mut self) -> Result<(), LoxError> {
    loop {
      let mut input = String::new();
      print!(">>> ");
      io::stdout().flush().unwrap();
      io::stdin().read_line(&mut input)?;
      if input.trim().is_empty() {
        continue;
      }
      match Self::run(&input, true) {
        Ok(_) => {}
        Err(_) => continue,
      }
    }
  }
}
