use std::fmt;
use std::rc::Rc;

use crate::number::Number;

#[derive(Clone)]
pub enum Expression {
    Combination(Vec<Expression>),
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(Number),
    Procedure(Vec<String>, Box<Expression>),
    BuiltinProcedure(Rc<dyn Fn(Vec<Expression>) -> Result<Expression, String>>),
    Void,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Combination(elements) => {
                let sub: Vec<String> = elements.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({})", sub.join(" "))
            }
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::StringLiteral(s) => write!(f, "\"{}\"", s),
            Expression::NumberLiteral(v) => write!(f, "{}", v),
            Expression::Procedure(_, _) => write!(f, "#procedure"),
            Expression::BuiltinProcedure(_) => write!(f, "#builtin"),
            Expression::Void => write!(f, ""),
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Combination(c1), Expression::Combination(c2)) => c1 == c2,
            (Expression::Identifier(i1), Expression::Identifier(i2)) => i1 == i2,
            (Expression::StringLiteral(s1), Expression::StringLiteral(s2)) => s1 == s2,
            (Expression::NumberLiteral(n1), Expression::NumberLiteral(n2)) => n1 == n2,
            (Expression::Void, Expression::Void) => true,
            _ => false
        }
    }
}