use std::collections::HashMap;

use crate::{Error, Result};
use crate::tokenize::Token;

macro_rules! prelude {
    ($($key:expr => $val:expr),*) => {
        {
            let mut hash_map: HashMap<String, fn(Vec<Token>) -> Token> = HashMap::new();
            $(
                hash_map.insert($key.to_string(), $val);
            )*
            hash_map
        }
    };
}

macro_rules! reduce {
    ($reducer:expr => $finalizer:expr) => {
        |args| reduce(args, $reducer, $finalizer).unwrap()
    };
}

impl From<Token> for f64 {
    fn from(token: Token) -> Self {
        match token {
            Token::Num(n) => n,
            Token::Bool(true) => 1.0,
            Token::Bool(false) => 0.0,
            Token::Str(s) if s.parse::<f64>().is_ok() => s.parse().unwrap(),
            _ => panic!()
        }
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token {
            Token::Str(s) => s,
            Token::Bool(b) => b.to_string(),
            Token::Num(n) => n.to_string(),
            _ => panic!()
        }
    }
}

impl From<Token> for bool {
    fn from(token: Token) -> Self {
        match token {
            Token::Bool(b) => b,
            _ => panic!()
        }
    }
}

/// A key value mapping of function names and the accompanying implementation.
pub type Prelude = HashMap<String, fn(Vec<Token>) -> Token>;

/// Returns a prelude (standard library) of often used functions.
pub fn get_prelude() -> Prelude {
    prelude!(
        "+" => reduce!(|a, b| a + b => Token::Num),
        "add" => reduce!(|a, b| a + b => Token::Num),
        "-" => reduce!(|a, b| a - b => Token::Num),
        "sub" => reduce!(|a, b| a - b => Token::Num),
        "*" => reduce!(|a, b| a * b => Token::Num),
        "mul" => reduce!(|a, b| a * b => Token::Num),
        "/" => reduce!(|a, b| a / b => Token::Num),
        "div" => reduce!(|a, b| a / b => Token::Num),
        "concat" => reduce!(|a, b| format!("{}{}", a, b) => Token::Str)
    )
}

/// Reduces the given vector of `Token`s  using the given `reducer` function.
/// The result is turned back into a vector using the `finalizer` function.
///
/// # Arguments
///
/// * `args` - The arguments passed to the function.
/// * `reducer` - A function used to reduce the args to a single value.
/// * `finalizer` - A function used to turn the result back into a `Token`.
///
/// # Errors
///
/// If the vector of args is empty, an error type will be returned.
///
/// # Panics
///
/// If one of the args can't be converted to a matching type, a panic occurs.
fn reduce<T, F, G>(args: Vec<Token>, reducer: F, finalizer: G) -> Result<Token>
    where
        T: From<Token>,
        F: Fn(T, T) -> T,
        G: Fn(T) -> Token
{
    // The `finalizer` could be replaced by adding `+ Into<Token>` to the type
    // constraints of `T`. We have decided against this approach for the
    // added readability.
    // The alternative implementation would require us to specify the type
    // encapsulated in the specific `Token` variant, so not much writing effort
    // would be saved anyways.
    args
        .into_iter()
        .map(|x| x.into())
        .reduce(reducer)
        .ok_or(Error::InvalidNumberOfArguments)
        .map(finalizer)
}
