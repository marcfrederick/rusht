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

pub type Prelude = HashMap<String, fn(Vec<Token>) -> Token>;

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

fn reduce<T, F, G>(args: Vec<Token>, reducer: F, finalizer: G) -> Result<Token>
    where
        T: From<Token>,
        F: Fn(T, T) -> T,
        G: Fn(T) -> Token
{
    args
        .into_iter()
        .map(|x| x.into())
        .reduce(reducer)
        .ok_or(Error::InvalidNumberOfArguments)
        .map(finalizer)
}
