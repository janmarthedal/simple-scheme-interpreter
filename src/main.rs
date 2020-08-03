use std::collections::HashMap;
use std::rc::Rc;

mod expression;
mod number;
mod parser;
mod tokenizer;
use expression::Expression;
use number::Number;

struct Environment {
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

fn eval(expr: &Expression, env: &mut Environment) -> Result<Expression, String> {
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

fn main() {
    let input = "(define (f a b) (+ a b))\n(f 1 2)\nf";
    let tokens = tokenizer::tokenize(input.chars());

    let mut global_env = Environment::new();

    global_env.push();
    global_env.insert(
        "+".to_string(),
        Expression::BuiltinProcedure(Rc::new(builtin_add)),
    );
    global_env.insert(
        "-".to_string(),
        Expression::BuiltinProcedure(Rc::new(builtin_sub)),
    );
    global_env.insert(
        "*".to_string(),
        Expression::BuiltinProcedure(Rc::new(builtin_mul)),
    );

    for expr in parser::Parser::new(tokens) {
        match expr {
            Ok(ex) => match eval(&ex, &mut global_env) {
                Ok(value) => {
                    println!("{}", value);
                }
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
