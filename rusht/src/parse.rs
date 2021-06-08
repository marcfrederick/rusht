use std::collections::VecDeque;

use thiserror::Error;

use crate::tokenize::Token;

#[derive(Error, Debug)]
pub enum Error {
    #[error("token stream ended unexpectedly")]
    UnexpectedEndOfTokenStream,
    #[error("encountered an unexpected closing parenthesis")]
    UnexpectedClosingParenthesis,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Exp {
    Atom(Token),
    List(Vec<Exp>),
}

pub fn parse(token_stream: Vec<Token>) -> Exp {
    let mut token_stream = VecDeque::from(token_stream);
    parse_it(&mut token_stream).unwrap()
}

fn parse_it(token_stream: &mut VecDeque<Token>) -> Result<Exp> {
    let token = token_stream.pop_front()
        .ok_or(Error::UnexpectedEndOfTokenStream)?;

    match token {
        Token::Paren('(') => {
            let mut l = vec![];
            while *token_stream.get(0).unwrap() != Token::Paren(')') {
                l.push(parse_it(token_stream)?);
            }
            Ok(Exp::List(l))
        }
        Token::Paren(')') => Err(Error::UnexpectedClosingParenthesis),
        atom => Ok(Exp::Atom(atom))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Token::*;

    macro_rules! test_parse {
        ($($name:ident: $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let out = parse($input);
                    assert_eq!(out, $expected);
                }
            )*
        };
    }

    test_parse!(
        test_empty: vec![Paren('('), Paren(')')] => Exp::List(vec![]),
        test_single: vec![Paren('('), Num(4.0), Paren(')')] => Exp::List(vec![Exp::Atom(Num(4.0))]),
        test_nested: vec![Paren('('), Num(4.0), Paren('('), Num(5.0), Str("foo".to_string()), Paren(')'), Paren(')')] => Exp::List(vec![Exp::Atom(Num(4.0)), Exp::List(vec![Exp::Atom(Num(5.0)), Exp::Atom(Str("foo".to_string()))])])
    );
}