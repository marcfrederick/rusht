use std::collections::HashMap;

use crate::parse::Expr;
use crate::prelude;
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

pub fn interpret(ast: Expr) -> Token {
    let env = hash_map!(
        "+" => prelude::add as fn(Vec<Token>) -> Token,
        "add" => prelude::add as fn(Vec<Token>) -> Token,
        "-" => prelude::sub as fn(Vec<Token>) -> Token,
        "sub" => prelude::sub as fn(Vec<Token>) -> Token,
        "*" => prelude::mul as fn(Vec<Token>) -> Token,
        "mul" => prelude::mul as fn(Vec<Token>) -> Token,
        "/" => prelude::div as fn(Vec<Token>) -> Token,
        "div" => prelude::div as fn(Vec<Token>) -> Token
    );

    match ast {
        Expr::Atom(token) => token,
        Expr::List(tokens) => {
            let tokens = tokens.iter()
                .map(|t| interpret(t.clone()))
                .collect::<Vec<_>>();

            let (func, args) = tokens.split_at(1);
            match func.get(0).unwrap() {
                Token::Ident(ident) => {
                    let ident = (*ident).as_str();
                    let func = env.get(ident).expect("function not found in env");
                    func(args.to_vec())
                }
                _ => panic!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_add() {
        let out = interpret(Expr::List(vec![
            Expr::Atom(Token::Ident(String::from("+"))),
            Expr::Atom(Token::Num(4.0)),
            Expr::Atom(Token::Num(5.0)),
            Expr::Atom(Token::Num(15.0)),
        ]));
        assert_eq!(out, Token::Num(24.0))
    }

    #[test]
    fn nested_add() {
        let out = interpret(Expr::List(vec![
            Expr::Atom(Token::Ident(String::from("+"))),
            Expr::Atom(Token::Num(4.0)),
            Expr::Atom(Token::Num(5.0)),
            Expr::List(vec![
                Expr::Atom(Token::Ident(String::from("+"))),
                Expr::Atom(Token::Num(10.0)),
                Expr::Atom(Token::Num(5.0)),
            ]),
        ]));
        assert_eq!(out, Token::Num(24.0))
    }
}
