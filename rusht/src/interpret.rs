use std::collections::HashMap;

use crate::parse::Expr;
use crate::tokenize::Token;

pub fn interpret(ast: Expr, env: &HashMap<String, fn(Vec<Token>) -> Token>) -> Token {
    match ast {
        Expr::Atom(token) => token,
        Expr::List(tokens) => {
            let tokens = tokens.iter()
                .map(|t| interpret(t.clone(), env))
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
    use crate::prelude;

    use super::*;

    #[test]
    fn single_add() {
        let mut env = HashMap::new();
        env.insert("+".to_string(), prelude::add as fn(Vec<Token>) -> Token);

        let out = interpret(Expr::List(vec![
            Expr::Atom(Token::Ident(String::from("+"))),
            Expr::Atom(Token::Num(4.0)),
            Expr::Atom(Token::Num(5.0)),
            Expr::Atom(Token::Num(15.0)),
        ]), &env);
        assert_eq!(out, Token::Num(24.0))
    }

    #[test]
    fn nested_add() {
        let mut env = HashMap::new();
        env.insert("+".to_string(), prelude::add as fn(Vec<Token>) -> Token);

        let out = interpret(Expr::List(vec![
            Expr::Atom(Token::Ident(String::from("+"))),
            Expr::Atom(Token::Num(4.0)),
            Expr::Atom(Token::Num(5.0)),
            Expr::List(vec![
                Expr::Atom(Token::Ident(String::from("+"))),
                Expr::Atom(Token::Num(10.0)),
                Expr::Atom(Token::Num(5.0)),
            ]),
        ]), &env);
        assert_eq!(out, Token::Num(24.0))
    }
}
