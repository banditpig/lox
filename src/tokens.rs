

use std::fmt::Debug;
use derive_new::new;


#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    // Literals.
    IDENTIFIER, STRING, NUMBER,
    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
    EOF,
    UNKNOWN,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    NUMBER , STRING , TRUE, FALSE, NIL,
}

    

#[derive(Debug, new, Clone)]
pub struct Token{
    pub tok_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}








