#![allow(dead_code)]
mod scanner;
mod tokens;

use scanner::*;

use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};
static mut HAD_ERROR: bool = false;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(1);
    } else if args.len() == 2 {
        run_file(args.get(1).unwrap());
    } else {
        run_prompt();
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
fn run(code: &String) {
    let mut s = Scanner::new(code.to_owned());
    s.scan_tokens();
    println!("{:?}", s);
    unsafe {
        
        HAD_ERROR = false;
    }
}
fn run_file(path: &String) {
    let code = fs::read_to_string(path).expect("Ooops");

    run(&code);
}
