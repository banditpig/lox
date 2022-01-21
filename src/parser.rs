use crate::{
    exprs::{Expr, LiteralValue},
    tokens::*,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(t:Vec<Token>) -> Self {
         Parser { tokens:t , current: 0}
    }

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.matches(&vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
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
        if self.is_at_end() {
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

    fn peek(&mut self) -> Token {
        return self.tokens[self.current].clone();
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.matches(&vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.matches(&vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }
        return self.previous();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().tok_type == TokenType::EOF;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.matches(&vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.matches(&vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator: operator,
                right: Box::new(right),
            };
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.matches(&vec![TokenType::FALSE]) {
            return Expr::Literal {
                literal: LiteralValue::Boolean(false),
            };
        }
        if self.matches(&vec![TokenType::TRUE]) {
            return Expr::Literal {
                literal: LiteralValue::Boolean(true),
            };
        }
        if self.matches(&vec![TokenType::NIL]) {
            return Expr::Literal {
                literal: LiteralValue::Null,
            };
        }
        if self.matches(&vec![TokenType::NUMBER]) {
            let xx = self.previous().literal;
            println!("NUMBER {:?}", xx);
            let n:f64 = xx.parse::<f64>().unwrap();//self.previous().literal.parse().unwrap();
            return Expr::Literal {
                literal: LiteralValue::Number(n),
            };
        }
        if self.matches(&vec![TokenType::STRING]) {
            let s = self.previous().literal;
            return Expr::Literal {
                literal: LiteralValue::String(s),
            };
        }
        if self.matches(&vec![TokenType::LEFT_PAREN]) {
           
            let expr = self.expression();
            self.consume(&TokenType::RIGHT_PAREN, "Expect ')' after expression.".to_string());
            return Expr::Grouping{ expression: Box::new(expr) }
        }
    
        return Expr::Literal {
            literal: LiteralValue::Boolean(false),
        };
    }

    fn consume(&mut self, tt: &TokenType, msg: String) -> Token{

        if self.check_type(tt){
            return self.advance()
        }else{
            panic!("{:?}", msg);
        }
    }


}
