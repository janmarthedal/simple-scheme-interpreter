use crate::expression::Expression;
use crate::environment::Environment;

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
