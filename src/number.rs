use std::fmt;
use std::ops::{Add, Mul, Sub, Div, Neg};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
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

pub struct ParseNumberError {}

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

impl Number {
    fn apply_binary_op<OpInt, OpFloat, R>(
        self,
        other: Self,
        op_int: OpInt,
        op_float: OpFloat,
    ) -> R
    where
        OpInt: Fn(i64, i64) -> R,
        OpFloat: Fn(f64, f64) -> R,
    {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => op_int(a, b),
            (Number::Float(a), Number::Float(b)) => op_float(a, b),
            (Number::Int(a), Number::Float(b)) => op_float(a as f64, b),
            (Number::Float(a), Number::Int(b)) => op_float(a, b as f64),
        }
    }
}

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Number::Int(v) => Number::Int(-v),
            Number::Float(v) => Number::Float(-v)
        }
    }
}

impl Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self.apply_binary_op(other, |a, b| Number::Int(a + b), |a, b| Number::Float(a + b))
    }
}

impl Sub for Number {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.apply_binary_op(other, |a, b| Number::Int(a - b), |a, b| Number::Float(a - b))
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        self.apply_binary_op(other, |a, b| Number::Int(a * b), |a, b| Number::Float(a * b))
    }
}

impl Div for Number {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self.apply_binary_op(other, |a, b| {
            if a % b == 0 {
                Self::Int(a / b)
            } else {
                Self::Float(a as f64 / b as f64)
            }
        }, |a, b| Number::Float(a / b))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.apply_binary_op(*other, |a, b| a == b, |a, b| a == b)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.apply_binary_op(*other, |a, b| a.partial_cmp(&b), |a, b| a.partial_cmp(&b))
    }
}
