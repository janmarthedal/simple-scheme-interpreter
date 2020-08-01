use std::fmt;

#[derive(Debug,Clone)]
pub enum Expression {
    Void,
    Combination(Vec<Expression>),
    Identifier(String),
    StringLiteral(String),
    FloatLiteral(f64),
    IntLiteral(i64),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Combination(elements) => {
                let sub: Vec<String> = elements.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({})", sub.join(" "))
            }
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::StringLiteral(s) => write!(f, "\"{}\"", s),
            Expression::FloatLiteral(v) => write!(f, "{:+.4e}", v),
            Expression::IntLiteral(v) => write!(f, "{}", v),
            Expression::Void => write!(f, "#void"),
        }
    }
}
