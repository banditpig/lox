use crate::{
    tokens::{
        Token,
        TokenType::{self, *},
    },
    HAD_ERROR,
};

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

#[derive(Debug)]
pub struct Scanner {
    source: String,
    line: usize,
    start: usize,
    current: usize,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(code: String) -> Self {
        Scanner {
            source: code,
            line: 0,
            start: 0,
            current: 0,
            tokens: Vec::new(),
        }
    }
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

        if self.is_alpha(c) {
            self.identifier();
            return;
        }

        match c {
            ' ' | '\r' | '\t' => (),
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
            '"' => self.string(),

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number(),

            _ => error(self.line, "Unexpected token."),
        }
    }
    fn add_token(&mut self, tt: TokenType) {
        self.add_token_1(tt, "".to_string());
    }

    fn add_token_1(&mut self, tt: TokenType, literal: String) {
        let txt = self.source.substring(self.start, self.current).to_string();
        let token = Token::new(tt, txt, literal, self.line);
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
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let literal = self
            .source
            .substring(self.start + 1, self.current - 1)
            .to_string();
        self.add_token_1(TokenType::STRING, literal)
    }

    fn is_digit(&self, ch: char) -> bool {
        match ch {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => return true,
            _ => return false,
        }
    }
    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let literal = self
            .source
            .substring(self.start, self.current)
            .to_string();

        self.add_token_1(TokenType::NUMBER, literal)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }
    fn is_alpha(&self, ch: char) -> bool {
        return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_';
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        return self.is_digit(ch) || self.is_alpha(ch);
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let ident = self
            .source
            .substring(self.start , self.current )
            .to_string();

        let mut tt = self.resolve_identifier(&ident);

        match tt {
            TokenType::UNKNOWN  => tt = TokenType::IDENTIFIER,
            _ => (),
        }

        self.add_token(tt);
        
    }

    fn resolve_identifier(&self, ident: &str) -> TokenType {
        println!("resolving {:?}", ident);
        match ident {
            "and"    => TokenType::AND,
            "class"  => TokenType::CLASS,
            "else"   => TokenType::ELSE,
            "false"  => TokenType::FALSE,
            "for"    => TokenType::FOR,
            "fun"    => TokenType::FUN,
            "if"     => TokenType::IF,
            "nil"    => TokenType::NIL,
            "or"     => TokenType::OR,
            "print"  => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "super"  => TokenType::SUPER,
            "this"   => TokenType::THIS,
            "true"   => TokenType::TRUE,
            "var"    => TokenType::VAR,
            "while"  => TokenType::WHILE,
            _        => TokenType::UNKNOWN,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let scnr = Scanner::new("".to_string());

        let mut tt = scnr.resolve_identifier("and");
        assert_eq!(TokenType::AND, tt);

        tt = scnr.resolve_identifier("class");
        assert_eq!(TokenType::CLASS, tt);

        tt = scnr.resolve_identifier("else");
        assert_eq!(TokenType::ELSE, tt);
        // etc

    }

    #[test]
    fn check_scanner(){
        let mut scnr = Scanner::new("-123 * 45.67".to_string());
        scnr.scan_tokens();
        for tt in scnr.tokens{
            println!("{:?}", tt) 
        }
       
    }


}
