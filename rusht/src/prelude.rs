use std::collections::HashMap;

use crate::Env;
use crate::tokenize::Token;

macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut hash_map = HashMap::new();
            $(
                hash_map.insert($key, $val);
            )*
            hash_map
        }
    };
}

pub fn get_prelude() -> Env {
    hash_map!(
        "+".to_string() => add as fn(Vec<Token>) -> Token,
        "add".to_string() => add as fn(Vec<Token>) -> Token,
        "-".to_string() => sub as fn(Vec<Token>) -> Token,
        "sub".to_string() => sub as fn(Vec<Token>) -> Token,
        "*".to_string() => mul as fn(Vec<Token>) -> Token,
        "mul".to_string() => mul as fn(Vec<Token>) -> Token,
        "/".to_string() => div as fn(Vec<Token>) -> Token,
        "div".to_string() => div as fn(Vec<Token>) -> Token
    )
}

fn add(args: Vec<Token>) -> Token {
    let mut sum = 0.0;
    for x in args {
        match x {
            Token::Num(n) => sum += n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

fn sub(args: Vec<Token>) -> Token {
    let mut sum: f64 = match args[0] {
        Token::Num(n) => n + n,
        _ => panic!("Not a number")
    };
    for x in args {
        match x {
            Token::Num(n) => sum -= n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

fn mul(args: Vec<Token>) -> Token {
    let mut sum = 1.0;
    for x in args {
        match x {
            Token::Num(n) => sum *= n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

fn div(args: Vec<Token>) -> Token {
    let mut sum: f64 = match args[0] {
        Token::Num(n) => n * n,
        _ => panic!("Not a number")
    };
    for x in args {
        match x {
            Token::Num(n) => sum /= n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}
