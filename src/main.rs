#![feature(vec_peek_mut)]
#![feature(map_try_insert)]

use crate::lox::Lox;
use std::env;

mod callable;
mod environment;
mod errors;
mod interpreter;
mod lexer;
mod lox;
mod macros;
mod parser;
mod resolver;
mod types;

fn main() {
  let args: Vec<String> = env::args().collect();

  let mut lox = Lox::new();

  if args.len() > 2 {
    println!("usage: lox [script], or lox (for repl)");
    return;
  } else if args.len() == 2 {
    lox.run_file(&args[1]).unwrap();
  } else {
    lox.run_prompt().unwrap();
  }
}
