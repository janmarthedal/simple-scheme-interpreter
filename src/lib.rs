pub mod eval {
    use std::collections::HashMap;
    use std::rc::Rc;

    use crate::number::Number;
    use crate::expression::Expression;

    pub struct Environment {
        stack: Vec<HashMap<String, Expression>>,
    }

    impl Environment {
        fn new() -> Self {
            Self { stack: Vec::new() }
        }
        fn push(&mut self) {
            self.stack.push(HashMap::new());
        }
        fn pop(&mut self) {
            self.stack.pop();
        }
        fn insert(&mut self, key: String, value: Expression) {
            self.stack.last_mut().unwrap().insert(key, value);
        }
        fn lookup(&self, key: &String) -> Option<&Expression> {
            self.stack.iter().rev().find_map(|m| m.get(key))
        }
    }

    pub fn eval(expr: &Expression, env: &mut Environment) -> Result<Expression, String> {
        match expr {
            Expression::Identifier(id) => match env.lookup(id) {
                Some(e) => Ok(e.clone()),
                None => Err(format!("Undefined symbol '{}'", id)),
            },
            Expression::Combination(elements) => {
                let mut elem_iter = elements.iter();
                if let Some(first_expr) = elem_iter.next() {
                    if let Expression::Identifier(id) = first_expr {
                        if id.as_str() == "define" {
                            let name_expr = elem_iter.next().ok_or("Invalid syntax".to_string())?;
                            let body_expr = elem_iter.next().ok_or("Invalid syntax".to_string())?;
                            if !elem_iter.next().is_none() {
                                return Err("Invalid syntax".to_string());
                            }
                            if let Expression::Identifier(name) = name_expr {
                                let body_value = eval(body_expr, env)?;
                                env.insert(name.clone(), body_value);
                            } else if let Expression::Combination(comb) = name_expr {
                                let mut ids = comb
                                    .iter()
                                    .map(|e| match e {
                                        Expression::Identifier(n) => Some(n.clone()),
                                        _ => None,
                                    })
                                    .collect::<Option<Vec<String>>>()
                                    .ok_or("Invalid syntax".to_string())?;
                                if ids.is_empty() {
                                    return Err("Invalid syntax".to_string());
                                }
                                let proc_name = ids.remove(0);
                                env.insert(
                                    proc_name,
                                    Expression::Procedure(ids, Box::new(body_expr.clone())),
                                );
                            } else {
                                return Err("Invalid syntax".to_string());
                            }
                            return Ok(Expression::Void);
                        }
                    }
                    let operand = eval(first_expr, env)?;
                    match operand {
                        Expression::BuiltinProcedure(p) => {
                            // run eval on arguments
                            let args = elem_iter
                                .map(|e| eval(e, env))
                                .collect::<Result<Vec<_>, String>>()?;
                            p(args)
                        }
                        Expression::Procedure(arg_names, body) => {
                            let args = elem_iter
                                .map(|e| eval(e, env))
                                .collect::<Result<Vec<_>, String>>()?;
                            if arg_names.len() != args.len() {
                                return Err("Wrong number of arguments".to_string());
                            }
                            env.push();
                            for (var, value) in arg_names.iter().zip(args.iter()) {
                                env.insert(var.clone(), value.clone());
                            }
                            let result = eval(body.as_ref(), env);
                            env.pop();
                            result
                        }
                        _ => Err(format!("Attempt to apply non-procedure '{}'", operand)),
                    }
                } else {
                    Err("Invalid syntax ()".to_string())
                }
            }
            other => Ok(other.clone()),
        }
    }

    fn builtin_add(args: Vec<Expression>) -> Result<Expression, String> {
        Ok(Expression::NumberLiteral(args.iter().try_fold(
            Number::from(0),
            |acc, v| match v {
                Expression::NumberLiteral(i) => Ok(acc + *i),
                _ => Err("Expecting number"),
            },
        )?))
    }

    fn builtin_mul(args: Vec<Expression>) -> Result<Expression, String> {
        Ok(Expression::NumberLiteral(args.iter().try_fold(
            Number::from(1),
            |acc, v| match v {
                Expression::NumberLiteral(i) => Ok(acc * *i),
                _ => Err("Expecting number"),
            },
        )?))
    }

    fn builtin_sub(args: Vec<Expression>) -> Result<Expression, String> {
        let mut arg_iter = args.iter();
        let first_num = match arg_iter.next() {
            Some(Expression::NumberLiteral(num)) => num,
            Some(_) => return Err("Expecting number".to_string()),
            None => return Err("Incorrect argument count in call (-)".to_string()),
        };
        Ok(Expression::NumberLiteral(arg_iter.try_fold(
            *first_num,
            |acc, v| match v {
                Expression::NumberLiteral(i) => Ok(acc - *i),
                _ => Err("Expecting number"),
            },
        )?))
    }

    pub fn create_root_environment() -> Environment {
        let mut root_env = Environment::new();

        root_env.push();

        root_env.insert(
            "+".to_string(),
            Expression::BuiltinProcedure(Rc::new(builtin_add)),
        );
        root_env.insert(
            "-".to_string(),
            Expression::BuiltinProcedure(Rc::new(builtin_sub)),
        );
        root_env.insert(
            "*".to_string(),
            Expression::BuiltinProcedure(Rc::new(builtin_mul)),
        );

        root_env
    }
}

pub mod parser {

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
}

pub mod expression {

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
}

pub mod tokenizer {

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
            let tokens = tokenize(input.chars()).collect::<Vec<Token>>();
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
}

pub mod number {

    use std::fmt;
    use std::ops::{Add, Mul, Sub};
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy)]
    pub enum Number {
        Int(i64),
        Float(f64),
    }

    impl From<i64> for Number {
        fn from(v: i64) -> Number {
            Self::Int(v)
        }
    }

    impl From<f64> for Number {
        fn from(v: f64) -> Number {
            Self::Float(v)
        }
    }

    pub struct ParseNumberError {}

    impl FromStr for Number {
        type Err = ParseNumberError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Ok(v) = s.parse::<i64>() {
                Ok(Number::Int(v))
            } else if let Ok(v) = s.parse::<f64>() {
                Ok(Number::Float(v))
            } else {
                Err(Self::Err {})
            }
        }
    }

    impl fmt::Display for Number {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Number::Int(v) => write!(f, "{}", v),
                Number::Float(v) => write!(f, "{:+.4e}", v),
            }
        }
    }

    impl Number {
        fn apply_binary_op<OpInt, OpFloat>(
            self,
            other: Self,
            op_int: OpInt,
            op_float: OpFloat,
        ) -> Self
        where
            OpInt: Fn(i64, i64) -> i64,
            OpFloat: Fn(f64, f64) -> f64,
        {
            match (self, other) {
                (Number::Int(a), Number::Int(b)) => Self::Int(op_int(a, b)),
                (Number::Float(a), Number::Float(b)) => Self::Float(op_float(a, b)),
                (Number::Int(a), Number::Float(b)) => Self::Float(op_float(a as f64, b)),
                (Number::Float(a), Number::Int(b)) => Self::Float(op_float(a, b as f64)),
            }
        }
    }

    impl Add for Number {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            self.apply_binary_op(other, |a, b| a + b, |a, b| a + b)
        }
    }

    impl Sub for Number {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            self.apply_binary_op(other, |a, b| a - b, |a, b| a - b)
        }
    }

    impl Mul for Number {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            self.apply_binary_op(other, |a, b| a * b, |a, b| a * b)
        }
    }

    impl PartialEq for Number {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Number::Int(a), Number::Int(b)) => *a == *b,
                (Number::Float(a), Number::Float(b)) => *a == *b,
                (Number::Int(a), Number::Float(b)) => *a as f64 == *b,
                (Number::Float(a), Number::Int(b)) => *a == *b as f64,
            }
        }
    }
}
