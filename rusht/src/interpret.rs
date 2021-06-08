use std::collections::HashMap;

use crate::parse::Ast;
use crate::prelude;
use crate::tokenize::Token;

macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut hash_map = HashMap::new();
            $( hash_map.insert($key, $val); )*
            hash_map
        }
    };
}

pub fn interpret(ast: Ast) -> Token {
    let env = hash_map! 
    (
        "+" => prelude::add,
        "add" => prelude::add,
        "-" => prelude::div,
        //"sub" => prelude::sub,
        //"*" => prelude::mul,
        //"mul" => prelude::mul,
        //"/" => prelude::div,
        //"div" => prelude::div
    );

    match ast {
        Ast::Atom(token) => token,
        Ast::List(tokens) => {
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
        let out = interpret(Ast::List(vec![
            Ast::Atom(Token::Ident(String::from("+"))),
            Ast::Atom(Token::Num(4.0)),
            Ast::Atom(Token::Num(5.0)),
            Ast::Atom(Token::Num(15.0)),
        ]));
        assert_eq!(out, Token::Num(24.0))
    }

    #[test]
    fn nested_add() {
        let out = interpret(Ast::List(vec![
            Ast::Atom(Token::Ident(String::from("+"))),
            Ast::Atom(Token::Num(4.0)),
            Ast::Atom(Token::Num(5.0)),
            Ast::List(vec![
                Ast::Atom(Token::Ident(String::from("+"))),
                Ast::Atom(Token::Num(10.0)),
                Ast::Atom(Token::Num(5.0)),
            ]),
        ]));
        assert_eq!(out, Token::Num(24.0))
    }


    #[test]
    fn nested_math() {
        let out = interpret(Ast::List(vec![
            Ast::Atom(Token::Ident(String::from("*"))),
            Ast::Atom(Token::Num(10.0)),
            Ast::List(vec![
                Ast::Atom(Token::Ident(String::from("/"))),
                Ast::Atom(Token::Num(10.0)),
                Ast::Atom(Token::Num(5.0)),
            ]),
        ]));
        assert_eq!(out, Token::Num(20.0))
    }
}
