use std::fmt;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug,Clone,Copy)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl From<i64> for Number {
    fn from(v: i64) -> Number {
        Self::Int(v)
    }
}

impl From<f64> for Number {
    fn from(v: f64) -> Number {
        Self::Float(v)
    }
}

pub struct ParseNumberError {
}

impl FromStr for Number {
    type Err = ParseNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<i64>() {
            Ok(Number::Int(v))
        } else if let Ok(v) = s.parse::<f64>() {
            Ok(Number::Float(v))
        } else {
            Err(Self::Err {})
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(v) => write!(f, "{}", v),
            Number::Float(v) => write!(f, "{:+.4e}", v),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Self::Int(a + b),
            (Number::Float(a), Number::Float(b)) => Self::Float(a + b),
            (Number::Int(a), Number::Float(b)) => Self::Float(a as f64 + b),
            (Number::Float(a), Number::Int(b)) => Self::Float(a + b as f64),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => *a == *b,
            (Number::Float(a), Number::Float(b)) => *a == *b,
            (Number::Int(a), Number::Float(b)) => *a as f64 == *b,
            (Number::Float(a), Number::Int(b)) => *a == *b as f64,
        }
    }
}
