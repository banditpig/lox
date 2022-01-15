use crate::{
    tokens::{
        Token,
        TokenType::{self, *},
    },
    HAD_ERROR,
};

use derive_new::new;
use std::fmt::Debug;
use substring::Substring;

fn error(line: usize, msg: &str) {
    report(line, "", msg);
}
fn report(line: usize, wher: &str, msg: &str) {
    println!("[line {} ] Error {} : {}", line, wher, msg);
    unsafe {
        HAD_ERROR = true;
    }
}

#[derive(Debug, new)]
pub struct Scanner {
    source: String,
    line: usize,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}
impl Scanner {
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let tok = Token::new(EOF, "".to_string(), "".to_string(), self.line);
        self.tokens.push(tok);
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            ' '  | 
            '\r' | 
            '\t' => (),
            '\n' => self.line = self.line + 1,
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }

            _ => error(self.line, "Unexpected token."),
        }
    }

    fn add_token(&mut self, tt: TokenType) {
        let txt = self.source.substring(self.start, self.current).to_string();

        let token = Token::new(tt, txt, "".to_string(), self.line);
        self.tokens.push(token);
    }
    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        return self.source.chars().nth(self.current - 1).unwrap();
    }
    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
    fn match_char(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != ch {
            return false;
        }
        self.current = self.current + 1;
        return true;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }
}