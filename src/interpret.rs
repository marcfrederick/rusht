use std::collections::HashMap;

use crate::parse::Ast;
use crate::tokenize::Token;

fn math(args: Vec<Token>) -> Token {
    let mut result = 0.0;
    for x in args {
        match x {
            '+' => result += x, 
            '-' => result -= x,
            '*' => result *= x,
            '/' => result /= x,
            _ => panic!("Not a number")
            
        }
    }
    Token::Num(result)
}



pub fn interpret(ast: Ast) -> Token {
    let mut mathmap = HashMap::new();

    match ast {
        Ast::Atom(token) => token,
        Ast::List(tokens) => {
            let charac = tokens.iter()
                .map(|t| interpret(t.clone()))
                .collect::<Vec<_>>();

            match charac.unwrap() {
                Token::Ident(ident) => {
                    let function = mathmap.insert(ident.to_string, math);
                },
                _ => panic!()
            }

        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_math() {
        let out = interpret(Ast::List(vec![
            Ast::Atom(Token::Ident(String::from("+"))),
            Ast::Atom(Token::Num(4.0)),
            Ast::Atom(Token::Num(5.0)),
            Ast::Atom(Token::Num(15.0)),
        ]));
        assert_eq!(out, Token::Num(24.0))
    }

    #[test]
    fn nested_math() {
        let out = interpret(Ast::List(vec![
            Ast::Atom(Token::Ident(String::from("-"))),
            Ast::Atom(Token::Num(6.0)),
            Ast::Atom(Token::Num(5.0)),
            Ast::Atom(Token::Ident(String::from("*"))),
            Ast::List(vec![
                Ast::Atom(Token::Ident(String::from("/"))),
                Ast::Atom(Token::Num(10.0)),
                Ast::Atom(Token::Num(5.0)),
            ]),
        ]));
        assert_eq!(out, Token::Num(2.0))
    }
}