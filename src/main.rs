use crate::lox::Lox;
use std::env;

mod expression;
mod lexer;
mod lox;
mod parser;
mod token;
mod interpreter;

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
