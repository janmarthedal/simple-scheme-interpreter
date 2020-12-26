use simple_scheme_interpreter::{environment::create_root_environment, eval::eval, parser::Parser, tokenizer::tokenize};

fn main() {
    let input = "(define (abs x) (cond ((> x 0) x) ((= x 0) 0) ((< x 0) (- x))))\n(abs -4)\n(abs 0)\n(abs 4)";
    let tokens = tokenize(input.chars());

    let mut env = create_root_environment();

    for expr in Parser::new(tokens) {
        match expr {
            Ok(ex) => match eval(&ex, &mut env) {
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
