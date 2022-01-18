use crate::tokens::Token;

pub trait Visitor<T> {
    fn visit_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> T;
    fn visit_unary(&mut self, op: &Token, right: &Expr) -> T;
    fn visit_grouping(&mut self, exp: &Expr) -> T;
    fn visit_literal(&mut self, g: &LiteralValue) -> T;
}


pub enum LiteralValue {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        literal: LiteralValue,
    },
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),

            Expr::Unary {
                 operator, 
                 right 
            } => visitor.visit_unary(operator, right),

            Expr::Grouping { 
                expression 
            } => visitor.visit_grouping(expression),

             Expr::Literal {
                  literal 
            } => visitor.visit_literal(literal)
        }
    }
}
