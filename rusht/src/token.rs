use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Paren(char),
    Num(f64),
    Str(String),
    Ident(String),
    Bool(bool),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Paren(x) => write!(f, "{}", x),
            Token::Num(x) => write!(f, "{}", x),
            Token::Str(x) => write!(f, "{}", x),
            Token::Ident(x) => write!(f, "{}", x),
            Token::Bool(x) => write!(f, "{}", x),
        }
    }
}

impl TryFrom<Token> for f64 {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Num(n) => Ok(n),
            Token::Bool(true) => Ok(1.0),
            Token::Bool(false) => Ok(0.0),
            Token::Str(s) if s.trim().parse::<f64>().is_ok() => Ok(s.trim().parse().unwrap()),
            _ => Err(Error::TypeError)
        }
    }
}

impl TryFrom<Token> for String {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Str(s) => Ok(s),
            Token::Bool(b) => Ok(b.to_string()),
            Token::Num(n) => Ok(n.to_string()),
            _ => Err(Error::TypeError)
        }
    }
}

impl TryFrom<Token> for bool {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Bool(b) => Ok(b),
            Token::Num(x) if x == 0.0 => Ok(false),
            Token::Num(_) => Ok(true),
            Token::Str(s) if ["true", "1"].contains(&s.as_str()) => Ok(true),
            Token::Str(s) if ["false", "0", ""].contains(&s.as_str()) => Ok(false),
            _ => Err(Error::TypeError)
        }
    }
}
