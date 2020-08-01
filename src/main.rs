mod expression;
mod parser;
mod tokenizer;
use expression::Expression;

fn eval(expr: &Expression) -> Result<Expression, String> {
    match expr {
        Expression::Identifier(_) => Err(format!("Not implemented")),
        Expression::Combination(elements) => {
            let mut elem_iter = elements.iter();
            if let Some(operand) = elem_iter.next() {
                match operand {
                    Expression::Identifier(id) => {
                        if id == "+" {
                            Ok(Expression::IntLiteral(elem_iter.try_fold(0i64, |acc, v| match v {
                                Expression::IntLiteral(i) => Ok(acc + i),
                                _ => Err("Expecting integer"),
                            })?))
                        } else {
                            Err(format!("Undefined operand '{}'", id))
                        }
                    }
                    _ => Err("Not implemented: Empty combination".to_string()),
                }
            } else {
                Err("Invalid syntax ()".to_string())
            }
        }
        other => Ok(other.clone()),
    }
}

fn main() {
    let input = "(+ 1 2 3 4 5)";
    let tokens = tokenizer::tokenize(input.chars());
    for expr in parser::Parser::new(tokens) {
        match expr {
            Ok(ex) => match eval(&ex) {
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
