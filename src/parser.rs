use std::fmt;
use crate::tokenizer::Token;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Expression {
    Expr(Vec<Expression>),
    Identifier(String),
    StringLiteral(String),
    FloatLiteral(f64),
    IntLiteral(i64),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Expr(e) => {
                let sub: Vec<String> = e.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({})", sub.join(" "))
            }
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::StringLiteral(s) => write!(f, "\"{}\"", s),
            Expression::FloatLiteral(v) => write!(f, "{:+.4e}", v),
            Expression::IntLiteral(v) => write!(f, "{}", v),
        }
    }
}

pub struct Parser<I: Iterator<Item = Token>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {

    pub fn new(iter: I) -> Self {
        Parser {
            iter: iter.peekable(),
        }
    }

    fn single(&mut self) -> Option<Result<Expression, String>> {
        match self.iter.next() {
            Some(token) => {
                match token {
                    Token::LParen => {
                        let mut items: Vec<Expression> = Vec::new();
                        loop {
                            match self.iter.peek() {
                                None => return Some(Err("Unexpected EOF".to_string())),
                                Some(Token::RParen) => {
                                    self.iter.next();
                                    break;
                                }
                                _ => {
                                    match self.single() {
                                        Some(Ok(expr)) => items.push(expr),
                                        err @ Some(Err(_)) => return err,
                                        None => return Some(Err("w00t".to_string()))
                                    }
                                }
                            }
                        }
                        Some(Ok(Expression::Expr(items)))
                    }
                    Token::RParen => Some(Err("Unexpected ')'".to_string())),
                    Token::Identifier(id) => Some(Ok(Expression::Identifier(id))),
                    Token::StringLiteral(st) => Some(Ok(Expression::StringLiteral(st))),
                    Token::FloatLiteral(v) => Some(Ok(Expression::FloatLiteral(v))),
                    Token::IntLiteral(v) => Some(Ok(Expression::IntLiteral(v)))
                }
            }
            None => None
        }
    }

}

impl<I: Iterator<Item = Token>> Iterator for Parser<I> {
    type Item = Result<Expression, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.single()
    }
}
