
use crate::{
    exprs::Expr,
    tokens::{Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.matches(&vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let  operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn check_type(&mut self, tt: &TokenType) -> bool {
        if self.is_at_end(){
            return false;
        }
        return &self.peek().tok_type == tt;
    }
    fn matches(&mut self, token_types: &Vec<TokenType>) -> bool {
        for tt in token_types {
            if self.check_type(tt) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn peek(&mut self) -> Token{
      
       return self.tokens[self.current].clone();
    }
    fn comparison(&self) -> Expr {
        todo!()
    }

    fn advance(&mut self) -> Token {
        if ! self.is_at_end() { 
            self.current = self.current + 1;
        }
        return self.previous();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().tok_type  == TokenType::EOF;
    }
}
