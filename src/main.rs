use crate::{
    expression::Expr,
    lexer::Lexer,
    parser::Parser,
    token::{Object, Token, TokenType},
    lox::Lox,
};
use std::{
    env, fmt, fs,
    io::{self, Write},
};

mod expression;
mod lexer;
mod parser;
mod token;
mod lox;

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
