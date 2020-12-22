use simple_scheme_interpreter::{environment::create_root_environment, eval::eval, parser::Parser, tokenizer::tokenize};

fn main() {
    let input = "(define (f a b) (+ a b))\n(f 1 2)\nf";
    let tokens = tokenize(input.chars());

    let mut root_env = create_root_environment();

    for expr in Parser::new(tokens) {
        match expr {
            Ok(ex) => match eval(&ex, &mut root_env) {
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
