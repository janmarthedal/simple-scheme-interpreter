use std::fmt;
use std::iter::Peekable;

enum Token {
    LParen,
    RParen,
    Identifier(String),
    StringLiteral(String),
    FloatLiteral(f64),
    IntLiteral(i64),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Identifier(id) => write!(f, "{}", id),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::FloatLiteral(v) => write!(f, "{}f", v),
            Token::IntLiteral(v) => write!(f, "{}i", v),
        }
    }
}

struct Tokenizer<I: Iterator<Item = char>> {
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
                        Some(c) => s.push(c)
                    }
                }
                Some(Token::StringLiteral(s))
            }
            Some(c) => {
                let mut id = c.to_string();
                loop {
                    match self.iter.peek() {
                        Some(&c2) => {
                            if c2.is_whitespace() || c2 == '(' || c2 == ')' {
                                break
                            } else {
                                id.push(c2);
                                self.iter.next();
                            }
                        }
                        None => break
                    }
                }
                if let Ok(v) = id.parse::<i64>() {
                    Some(Token::IntLiteral(v))
                } else if let Ok(v) = id.parse::<f64>() {
                    Some(Token::FloatLiteral(v))
                } else {
                    Some(Token::Identifier(id))
                }
            }
            None => None,
        }
    }
}

fn tokenize<I: Iterator<Item = char>>(iter: I) -> Tokenizer<I> {
    Tokenizer {
        iter: iter.peekable(),
    }
}

fn test(s: &str) -> Vec<Token> {
    tokenize(s.chars()).collect()
}

fn main() {
    let tokens = test("(if (= foo 1.2) 12.3 \"bar\")");
    println!(
        "{}",
        tokens
            .iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join(" ")
    );
}
