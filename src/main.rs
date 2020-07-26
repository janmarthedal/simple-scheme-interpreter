mod tokenizer;

fn main() {
    let input = "(quote (testing 1 (2.0) -3.14e159))";
    let tokens = tokenizer::tokenize(input.chars());
    println!(
        "{}",
        tokens
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join(" ")
    );
}
