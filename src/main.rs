#![allow(dead_code)]
mod exprs;
mod parser;
mod scanner;
mod tokens;

use parser::*;
use scanner::*;

use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use crate::exprs::AstPrinter;
static mut HAD_ERROR: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args.get(1).unwrap()),
        _ => {
            println!("Usage: jlox [script]");
            exit(1);
        }
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();

        stdin.read_line(input).unwrap();
        if input.eq(&"\n") {
            break;
        }
        input.pop();
        run(input);
    }
}
fn run(code: &str) {
    let mut s = Scanner::new(code.to_owned());

    s.scan_tokens();
    println!("{:?}", s.tokens);

    let mut printer = AstPrinter;
    let mut p = Parser::new(s.tokens);
    let expr = p.expression();

    let txt = printer.print(&expr);
    println!("{}", txt);

    unsafe {
        HAD_ERROR = false;
    }
}
fn run_file(path: &str) {
    let code = fs::read_to_string(path).expect("Ooops");

    run(&code);
}
