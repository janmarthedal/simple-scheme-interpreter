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
    /*fn pop(&mut self) {
        self.stack.pop();
    }*/
    fn insert(&mut self, key: String, value: Expression) {
        self.stack.last_mut().unwrap().insert(key, value);
    }
    fn lookup(&self, key: &String) -> Option<&Expression> {
        self.stack.iter().rev().find_map(|m| m.get(key))
    }
}

fn eval(expr: &Expression, env: &Environment) -> Result<Expression, String> {
    match expr {
        Expression::Identifier(_) => Err(format!("Not implemented")),
        Expression::Combination(elements) => {
            let mut elem_iter = elements.iter();
            if let Some(operand) = elem_iter.next() {
                match operand {
                    Expression::Identifier(id) => match env.lookup(id) {
                        Some(Expression::BuiltinProcedure(p)) => {
                            let args = elem_iter
                                .map(|e| eval(e, env))
                                .collect::<Result<Vec<_>, String>>()?;
                            let result = p(args)?;
                            Ok(result)
                        }
                        Some(e) => Err(format!("Attempt to apply non-procedure '{}'", e)),
                        None => Err(format!("Undefined operand '{}'", id)),
                    },
                    _ => Err("Expected procedure operand".to_string()),
                }
            } else {
                Err("Invalid syntax ()".to_string())
            }
        }
        other => Ok(other.clone()),
    }
}

fn plus(args: Vec<Expression>) -> Result<Expression, String> {
    Ok(Expression::NumberLiteral(args.iter().try_fold(
        Number::from(0),
        |acc, v| match v {
            Expression::NumberLiteral(i) => Ok(acc + *i),
            _ => Err("Expecting integer"),
        },
    )?))
}

fn main() {
    let input = "(- 1 2 3 4 5)";
    let tokens = tokenizer::tokenize(input.chars());

    let mut global_env = Environment::new();

    global_env.push();
    global_env.insert("+".to_string(), Expression::BuiltinProcedure(Rc::new(plus)));

    for expr in parser::Parser::new(tokens) {
        match expr {
            Ok(ex) => match eval(&ex, &global_env) {
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
