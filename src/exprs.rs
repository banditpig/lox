
use crate::tokens::*;
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

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String{
        expr.accept(self)

    }
    fn parenthesize(&mut self, name: String, exprs: Vec<&Expr>) -> String{ 
        let mut rslt = String::new();
        rslt.push_str("(");
        rslt.push_str(&name);
        
        for e in exprs{
            rslt.push_str(" ");
            rslt.push_str(&e.accept(self))
        }
        rslt.push_str(")");
        return rslt;
    }
   
}
impl Visitor<String> for AstPrinter{
    fn visit_binary(&mut self, left: &Expr, op: &Token, right: &Expr) -> String {
        return self.parenthesize(op.lexeme.clone(), vec![left, right]);
    }

    fn visit_unary(&mut self, op: &Token, right: &Expr) -> String {
        //return parenthesize(expr.operator.lexeme, expr.right);
        return self.parenthesize(op.lexeme.to_string(), vec![right])
    }

    fn visit_grouping(&mut self, exp: &Expr) -> String {
        return self.parenthesize("Group".to_string(), vec![exp])

    }

    fn visit_literal(&mut self, exp: &LiteralValue) -> String {
        return match exp{
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Null => "Null".to_string(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.to_string(),
        };
    }
}


#[test]
fn printer_test(){
    let li = Expr::Literal {
        literal: LiteralValue::Number(123.0),
    };
    let t = Token {
        tok_type: TokenType::MINUS,
        lexeme: "-".to_string(),
        literal: "".to_string(),
        line: 1,
    };
    let unary_op = Box::new(Expr::Unary {
        operator: t,
        right: Box::new(li),
    });

    let mul = Token {
        tok_type: TokenType::STAR,
        lexeme: "*".to_string(),
        literal: "".to_string(),
        line: 1,
    };


    let grp = Box::new(Expr::Grouping {
        expression: Box::new(Expr::Literal {
            literal: LiteralValue::Number(45.67),
        }),
    });

    
    let bin_expr = Expr::Binary {
        left: unary_op,
        operator: mul,
        right: grp,
    };

    let mut printer = AstPrinter;
    let txt = printer.print(&bin_expr);
    println!("{}", txt);
    assert_eq!(
        txt,
        "(* (- 123) (Group 45.67))"
    );

  
}