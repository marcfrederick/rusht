use std::iter::Peekable;
use std::str::Chars;

use peeking_take_while::PeekableExt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Atom(String),
    Num(f64),
    Brace(char),
}

fn take_number(it: &mut Peekable<Chars>) -> f64 {
    it.peeking_take_while(|&c| ('0'..='9').contains(&c) || c == '.')
        .collect::<String>()
        .parse::<f64>()
        .unwrap()
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = vec![];

    let mut it = input.chars().peekable();
    while let Some(&next) = it.peek() {
        match next {
            '0'..='9' => tokens.push(Token::Num(take_number(&mut it))),
            '(' | ')' => tokens.push(Token::Brace(it.next().unwrap())),
            _ => unimplemented!()
        };
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_tokenize {
        ($($name:ident: $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let out = tokenize($input.to_string());
                    assert_eq!(out, $expected);
                }
            )*
        };
    }

    test_tokenize!(
        tokenize_empty: "()" => vec![Token::Brace('('), Token::Brace(')')],
        tokenize_integer: "1" => vec![Token::Num(1.0)],
        tokenize_long_integer: "1234" => vec![Token::Num(1234.0)],
        tokenize_float: "1.234" => vec![Token::Num(1.234)],
        tokenize_integer_expr: "(1)" => vec![Token::Brace('('), Token::Num(1.0), Token::Brace(')')]
    );

    // tokenize_one: "(foo)" => vec![Token::Brace('('), Token::Atom("foo".to_string()), Token::Brace(')')]
    // tokenize_atom: "atom" => vec![Token::Atom("atom".to_string())],
    // tokenize_two: "(foo bar)" => vec!["(", "foo", "bar", ")"],
    // tokenize_three: "(foo bar baz)" => vec!["(", "foo", "bar", "baz", ")"],
    // tokenize_add: "(+ 1 2)" => vec!["(", "+", "1", "2", ")"],
    // tokenize_nested: "(+ 1 (- 3 2))" => vec!["(", "+", "1", "(", "-", "3", "2", ")", ")"]
}
