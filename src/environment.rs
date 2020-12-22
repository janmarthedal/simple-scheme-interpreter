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
    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }
    pub fn pop(&mut self) {
        self.stack.pop();
    }
    pub fn insert(&mut self, key: String, value: Expression) {
        self.stack.last_mut().unwrap().insert(key, value);
    }
    pub fn lookup(&self, key: &String) -> Option<&Expression> {
        self.stack.iter().rev().find_map(|m| m.get(key))
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
