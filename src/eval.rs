use crate::environment::Environment;
use crate::expression::Expression;

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
                    if id == "define" {
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
                    } else if id == "cond" {
                        for clause in elem_iter {
                            match clause {
                                Expression::Combination(e) => {
                                    if e.len() != 2 {
                                        return Err("Expecting pair as cond clause".to_string());
                                    }
                                    let predicate = eval(&e[0], env)?;
                                    match predicate {
                                        Expression::BooleanLiteral(false) => {}
                                        _ => {
                                            let value = eval(&e[1], env)?;
                                            return Ok(value);
                                        }
                                    }
                                }
                                _ => return Err("Invalid syntax".to_string()),
                            }
                        }
                        // TODO return unspecified
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

#[cfg(test)]
mod test {
    use super::eval;
    use crate::environment::create_root_environment;
    use crate::expression::Expression;
    use crate::number::Number;
    use crate::parser::Parser;
    use crate::tokenizer::tokenize;

    fn single_expr_eq(input: &str, expected: Expression) {
        let tokens = tokenize(input.chars());
        let mut root_env = create_root_environment();
        let parser = Parser::new(tokens);
        let results: Result<Vec<Expression>, String> =
            parser.map(|e| eval(&e.unwrap(), &mut root_env)).collect();
        assert_eq!(results.unwrap().last().unwrap(), &expected);
    }

    fn int_expr(v: i64) -> Expression {
        Expression::NumberLiteral(Number::from(v))
    }

    #[test]
    fn literal_number() {
        single_expr_eq("486", int_expr(486));
    }

    #[test]
    fn simple_add() {
        single_expr_eq("(+ 137 349)", int_expr(486));
    }

    #[test]
    fn simple_sub() {
        single_expr_eq("(- 1000 334)", int_expr(666));
    }

    #[test]
    fn unary_minus() {
        single_expr_eq("(- 42)", int_expr(-42));
    }

    #[test]
    fn simple_mul() {
        single_expr_eq("(* 5 99)", int_expr(495));
    }

    #[test]
    fn simple_div() {
        single_expr_eq("(/ 10 5)", int_expr(2));
    }

    #[test]
    fn complex_arith() {
        single_expr_eq("(+ (* 3 (+ (* 2 4) (+ 3 5))) (+ (- 10 7) 6))", int_expr(57));
    }

    #[test]
    fn simple_define() {
        single_expr_eq("(define size 2) size", int_expr(2));
    }

    #[test]
    fn more_defines() {
        single_expr_eq(
            "(define pi 3.14159) (define radius 10) (define circumference (* 2 pi radius)) circumference", 
            Expression::NumberLiteral(Number::from(62.8318))
        );
    }

    #[test]
    fn simple_procedure() {
        single_expr_eq("(define (square x) (* x x)) (square 21)", int_expr(441));
    }

    #[test]
    fn more_procedures() {
        single_expr_eq(
            "(define (square x) (* x x)) (define (sum-of-squares x y) (+ (square x) (square y))) (define (f a) (sum-of-squares (+ a 1) (* a 2))) (f 5)",
            int_expr(136)
        );
    }

    #[test]
    fn literal_true() {
        single_expr_eq("#t", Expression::BooleanLiteral(true));
    }

    #[test]
    fn literal_false() {
        single_expr_eq("#f", Expression::BooleanLiteral(false));
    }
}
