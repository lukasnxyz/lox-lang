use crate::{errors::LoxError, interpreter::Interpreter, lexer::Lexer, parser::Parser};
use std::{fs, io, io::Write};

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

        let interpreter = Interpreter {};
        let val = expression.accept(&interpreter).unwrap();

        println!("{}", val);
        //println!("{}", expression);

        Ok(())
    }

    pub fn run_file(&self, path: &str) -> Result<(), LoxError> {
        let source = fs::read_to_string(path)?;
        Self::run(&source)?;
        if self.had_error {
            return Err(LoxError::Error);
        }
        Ok(())
    }

    // TODO: ctrl-c does nothing, ctrl-d quits
    // TODO: up and down arrow for history
    // TODO: left and right arrow for editing text
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
}
