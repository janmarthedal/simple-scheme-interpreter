use std::iter::Peekable;

use crate::expression::Expression;
use crate::tokenizer::Token;

pub struct Parser<I: Iterator<Item = Token>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(iter: I) -> Self {
        Parser {
            iter: iter.peekable(),
        }
    }

    fn single(&mut self) -> Result<Option<Expression>, String> {
        match self.iter.next() {
            Some(token) => match token {
                Token::LParen => {
                    let mut elements: Vec<Expression> = Vec::new();
                    loop {
                        match self.iter.peek() {
                            Some(Token::RParen) => {
                                self.iter.next();
                                break;
                            }
                            None => return Err("Unexpected EOF".to_string()),
                            _ => match self.single()? {
                                Some(expr) => elements.push(expr),
                                None => panic!("w00t"),
                            },
                        }
                    }
                    Ok(Some(Expression::Combination(elements)))
                }
                Token::RParen => Err("Unexpected ')'".to_string()),
                Token::Identifier(id) => Ok(Some(Expression::Identifier(id))),
                Token::StringLiteral(st) => Ok(Some(Expression::StringLiteral(st))),
                Token::NumberLiteral(v) => Ok(Some(Expression::NumberLiteral(v))),
            },
            None => Ok(None),
        }
    }
}

impl<I: Iterator<Item = Token>> Iterator for Parser<I> {
    type Item = Result<Expression, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.single().transpose()
    }
}
