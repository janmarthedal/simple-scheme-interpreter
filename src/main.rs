mod tokenizer;

fn test(s: &str) -> Vec<tokenizer::Token> {
    tokenizer::tokenize(s.chars()).collect()
}

fn main() {
    let tokens = test("(if (= foo 1.2) 12.3 \"bar\")");
    println!(
        "{}",
        tokens
            .iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join(" ")
    );
}
