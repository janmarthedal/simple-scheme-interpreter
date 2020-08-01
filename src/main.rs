mod tokenizer;
mod parser;

fn main() {
    let input = "(quote (testing 1 (2.0) -3.14e159)))";
    let tokens = tokenizer::tokenize(input.chars());
    /*println!(
        "{}",
        tokens
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join(" ")
    );*/
    for expr in parser::Parser::new(tokens) {
        match expr {
            Ok(ex) => println!("{}", ex),
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
