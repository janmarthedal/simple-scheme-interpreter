use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;

use crate::number::Number;

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(Number),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Identifier(id) => write!(f, "{}", id),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::NumberLiteral(v) => write!(f, "{}", v),
        }
    }
}

pub struct Tokenizer<I: Iterator<Item = char>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.peek() {
                Some(c) if c.is_whitespace() => {
                    self.iter.next();
                }
                _ => break,
            }
        }

        match self.iter.next() {
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            Some('"') => {
                let mut s = String::new();
                loop {
                    match self.iter.next() {
                        Some('"') | None => break,
                        Some(c) => s.push(c),
                    }
                }
                Some(Token::StringLiteral(s))
            }
            Some(c) => {
                let mut id = c.to_string();
                loop {
                    match self.iter.peek() {
                        Some('(') | Some(')') => break,
                        Some(c2) if c2.is_whitespace() => break,
                        Some(c2) => {
                            id.push(*c2);
                            self.iter.next();
                        }
                        None => break,
                    }
                }
                if let Ok(v) = Number::from_str(id.as_str()) {
                    Some(Token::NumberLiteral(v))
                } else {
                    Some(Token::Identifier(id))
                }
            }
            None => None,
        }
    }
}

pub fn tokenize<I: Iterator<Item = char>>(iter: I) -> Tokenizer<I> {
    Tokenizer {
        iter: iter.peekable(),
    }
}

#[cfg(test)]
mod test {
    use super::{tokenize, Token};
    use crate::number::Number;

    #[test]
    fn basics() {
        let input = "(quote (testing 1 (2.0) -3.14e159))";
        let tokens: Vec<Token> = tokenize(input.chars()).collect();
        assert_eq!(
            vec![
                Token::LParen,
                Token::Identifier("quote".to_string()),
                Token::LParen,
                Token::Identifier("testing".to_string()),
                Token::NumberLiteral(Number::from(1)),
                Token::LParen,
                Token::NumberLiteral(Number::from(2)),
                Token::RParen,
                Token::NumberLiteral(Number::from(-3.14e159)),
                Token::RParen,
                Token::RParen
            ],
            tokens
        );
    }
}
