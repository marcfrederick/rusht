//! The expression type of the Rusht language. Different expression types are
//! represented by the different variants of the `Expr` enum.
//!
//! This module also implements conversion methods from Rusht to Rust types
//! and vice versa. Conversions from Rusht to Rust are implemented using the
//! `TryInto` trait and perform type coercion. This means, that an
//! `Expr::Str("1")` could be coerced to the `String` `"1"`, the `f64` `1.0`,
//! the `bool` `true`, ... depending on context.
//!
//! As not every Rusht type can be converted to every Rust type, this
//! conversion is not guaranteed to succeed and is thus implemented using the
//! `TryFrom` trait. Conversely, conversion in the other direction is
//! guaranteed to succeed and is thus implemented using the `Into` trait.

use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::tokenize::Token;
use crate::{Error, Result};

/// Lambda is a struct representing a single lambda expression.
#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    /// The names of the arguments of the lambda expression. On invocation of
    /// the lambda, these will be defined as variables corresponding to the
    /// passed values.
    pub args: Vec<String>,

    /// The body of the lambda. This body will be interpreted upon invocation
    /// of the lambda expression.
    pub body: Box<Expr>,
}

/// An expression in the "Rusht" language.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Num(f64),
    Str(String),
    Ident(String),
    Bool(bool),
    List(Vec<Expr>),
    Func(fn(Vec<Expr>) -> Result<Expr>),
    Lambda(Lambda),
}

/// This implementation of the `Display` trait determines how the different
/// expressions are displayed in the REPL.
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Num(x) => write!(f, "{}", x),
            Expr::Str(x) => write!(f, "\"{}\"", x),
            Expr::Ident(x) => write!(f, "{}", x),
            Expr::Bool(x) => write!(f, "{}", x),
            Expr::Lambda(Lambda { args, body }) => {
                write!(f, "\u{3bb} {} -> {}", stringify(args), body.to_string())
            }
            Expr::List(list) => write!(f, "{}", stringify(list)),
            Expr::Func(_) => write!(f, "prelude function"),
        }
    }
}

/// Returns the string representation of a given slice.
///
/// # Arguments
///
/// * `xs` - The slice for which to generate a string representation.
fn stringify<T>(xs: &[T]) -> String
where
    T: ToString,
{
    format!(
        "({})",
        xs.iter().map(T::to_string).collect::<Vec<_>>().join(" ")
    )
}

impl From<f64> for Expr {
    fn from(n: f64) -> Self {
        Expr::Num(n)
    }
}

impl From<String> for Expr {
    fn from(s: String) -> Self {
        Expr::Str(s)
    }
}

impl From<bool> for Expr {
    fn from(b: bool) -> Self {
        Expr::Bool(b)
    }
}

impl TryFrom<Token> for Expr {
    type Error = Error;

    fn try_from(value: Token) -> Result<Self> {
        match value {
            Token::Num(x) => Ok(Expr::Num(x)),
            Token::Str(x) => Ok(Expr::Str(x)),
            Token::Ident(x) => Ok(Expr::Ident(x)),
            Token::Bool(x) => Ok(Expr::Bool(x)),
            Token::Paren(_) => Err(Error::UnexpectedType),
        }
    }
}

impl TryFrom<Expr> for f64 {
    type Error = Error;

    fn try_from(expr: Expr) -> Result<Self> {
        match expr {
            Expr::Num(n) => Ok(n),
            Expr::Bool(true) => Ok(1.0),
            Expr::Bool(false) => Ok(0.0),
            Expr::Str(s) => s.trim().parse().map_err(|_| Error::UnexpectedType),
            _ => Err(Error::UnexpectedType),
        }
    }
}

impl TryFrom<Expr> for String {
    type Error = Error;

    fn try_from(expr: Expr) -> Result<Self> {
        match expr {
            Expr::Str(s) => Ok(s),
            Expr::Bool(b) => Ok(b.to_string()),
            Expr::Num(n) => Ok(n.to_string()),
            _ => Err(Error::UnexpectedType),
        }
    }
}

impl TryFrom<Expr> for bool {
    type Error = Error;

    fn try_from(expr: Expr) -> Result<Self> {
        match expr {
            Expr::Bool(b) => Ok(b),
            Expr::Num(x) if x == 0.0 => Ok(false),
            Expr::Num(_) => Ok(true),
            Expr::Str(s) if ["true", "1"].contains(&s.trim()) => Ok(true),
            Expr::Str(s) if ["false", "0", ""].contains(&s.trim()) => Ok(false),
            _ => Err(Error::UnexpectedType),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stringify_string_slice() {
        assert_eq!(stringify(&["foo", "bar"]), "(foo bar)")
    }

    #[test]
    fn test_stringify_expr_slice() {
        assert_eq!(
            stringify(&[
                Expr::Num(5.0),
                Expr::Str("foo".to_string()),
                Expr::List(vec![Expr::Ident("bar".to_string()), Expr::Bool(true),]),
                Expr::Lambda(Lambda {
                    args: vec!["a".to_string()],
                    body: Box::from(Expr::List(vec![
                        Expr::Ident("+".to_string()),
                        Expr::Ident("a".to_string()),
                        Expr::Num(1.0)
                    ])),
                })
            ]),
            "(5 \"foo\" (bar true) \u{3bb} (a) -> (+ a 1))"
        )
    }
}
