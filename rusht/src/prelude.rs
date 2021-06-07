use crate::tokenize::Token;
use std::borrow::Borrow;

pub fn add(args: Vec<Token>) -> Token {
    let mut sum = 0.0;
    for x in args {
        match x {
            Token::Num(n) => sum += n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

pub fn sub(args: Vec<Token>) -> Token {
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

pub fn mul(args: Vec<Token>) -> Token {
    let mut sum = 1.0;
    for x in args {
        match x {
            Token::Num(n) => sum *= n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

pub fn div(args: Vec<Token>) -> Token {
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
