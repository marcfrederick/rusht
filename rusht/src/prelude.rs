use std::collections::HashMap;

use crate::Env;
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

pub fn get_prelude() -> Env {
    prelude!(
        "+" => add,
        "add" => add,
        "-" => sub,
        "sub" => sub,
        "*" => mul,
        "mul" => mul,
        "/" => div,
        "div" => div
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
