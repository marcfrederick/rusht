/// As we know the TokenStream only takes Tokens which are
/// a roundup of Atoms (= Operaters), Parantheses (= Braces)
/// and Numbers.
/// Here we define these types with the special extras.

use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::{Error, Result};

/// Represent the datatypes that are defines as a Token.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    /// The braces.
    Paren(char),
    /// The given numbers.
    Num(f64),
    /// The input or a normal string.
    Str(String),
    /// The operation which then calls the function.
    Ident(String),
    /// For returning true or false.
    Bool(bool),
}

/// To use the '{}' the trait fmt::Display has to be implemented.
impl Display for Token {
    /// Also the trait need the needed signature which is represented
    /// with Formatter 'fmt'.
    /// If the written Token in the terminal is true it will be
    /// printed to the terminal.
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

/// To give back the Number datatype.
impl From<f64> for Token {
    fn from(n: f64) -> Self {
        Token::Num(n)
    }
}


/// To give back the String datatype.
impl From<String> for Token {
    fn from(s: String) -> Self {
        Token::Str(s)
    }
}


/// To give back the Boolean datatype.
impl From<bool> for Token {
    fn from(b: bool) -> Self {
        Token::Bool(b)
    }
}


/// Implementing datatypes for more interaction as
/// a Number as well with checking the coercion error.
impl TryFrom<Token> for f64 {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Num(n) => Ok(n),
            Token::Bool(true) => Ok(1.0),
            Token::Bool(false) => Ok(0.0),
            Token::Str(s) => s.trim().parse().map_err(|_| Error::CouldNotCoerceType),
            _ => Err(Error::CouldNotCoerceType)
        }
    }
}

/// Implementing datatypes for more interaction as
/// a String as well with checking the coercion error.
impl TryFrom<Token> for String {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Str(s) => Ok(s),
            Token::Bool(b) => Ok(b.to_string()),
            Token::Num(n) => Ok(n.to_string()),
            _ => Err(Error::CouldNotCoerceType)
        }
    }
}


/// For interaction with the other datatypes they are
/// implemented to be used as a Boolean as well.
impl TryFrom<Token> for bool {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Bool(b) => Ok(b),
            Token::Num(x) if x == 0.0 => Ok(false),
            Token::Num(_) => Ok(true),
            Token::Str(s) if ["true", "1"].contains(&s.trim()) => Ok(true),
            Token::Str(s) if ["false", "0", ""].contains(&s.trim()) => Ok(false),
            _ => Err(Error::CouldNotCoerceType)
        }
    }
}
