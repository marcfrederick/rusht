use std::collections::HashMap;

use crate::parse::Ast;
use crate::tokenize::Token;

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

pub fn interpret(ast: Ast) -> Token {
    let mut env = HashMap::new();
    env.insert(String::from("+"), add);

    match ast {
        Ast::Atom(token) => token,
        Ast::List(tokens) => {
            let tokens = tokens.iter()
                .map(|t| interpret(t.clone()))
                .collect::<Vec<_>>();

            let (func, args) = tokens.split_at(1);
            match func.get(0).unwrap() {
                Token::Ident(ident) => {
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
}
