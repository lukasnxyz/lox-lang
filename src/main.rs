use crate::lox::Lox;
use std::env;

mod errors;
mod expression;
mod interpreter;
mod lexer;
mod lox;
mod parser;
mod token;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("usage: lox [script], or lox (for repl)");
        return;
    } else if args.len() == 2 {
        lox.run_file(&args[0]).unwrap();
    } else {
        lox.run_prompt().unwrap();
    }
}
