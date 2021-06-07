use std::collections::HashMap;

use crate::parse::Ast;
use crate::tokenize::Token;


fn add(args: Vec<f64>) -> Token {
    let mut result = 0.0;
    for x in nums {
        match x {
            Token::Num(n) => result += n,
            _ => panic!()
        }
    }
    Token::Num(result)
}


pub fn interpret(ast: Ast) -> Token 
{

    let mut nums = vec![];

    match ast {
        Ast::Atom(atom) => atom,
        Ast::List(list) => {
            let tokens = list.iter()
                .map(|x| interpret(x.clone()))
                .collect::<Vec<_>>();

            match tokens {
                Token::Num(num) => {
                    nums.push(num);
                },
                Token::Ident(ident) => {
                    match ident.as_str() {
                        "+" => add(nums),
                        "-" => sub(nums),
                        "*" => mul(nums),
                        "/" => div(nums),
                        _ => panic!("No math operation\n")
                    }
                }
                _ => panic!()
            }
        }
    }
}