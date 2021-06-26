/// This is our Lisp Interpreter's first step:
/// Here we pass our terminal input into a TokenStream.
/// This gives us the opportunity to first of all identify our input's data types.
/// And secondly put everything together in a tokenstream for passing it to the next step.
use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

/// Takes the input from our terminal and checks each char with allocating it to the right function.
/// In the end we have each input's type which we pass to the Parser.
///
/// # Arguments
///
/// * `input` - The passed input.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut it = input.chars().peekable();
    while let Some(c) = it.peek() {
        match c {
            '(' | ')' => tokens.push(Token::Paren(it.next().unwrap())),
            '0'..='9' => tokens.push(take_number(&mut it)),
            '"' => tokens.push(take_str(&mut it)),
            _ if c.is_whitespace() => {
                it.next();
            }
            _ => tokens.push(take_ident_or_bool(&mut it)),
        };
    }

    tokens
}

/// Takes a single number from the characters. Numbers are made up of the
/// numerals from 0 to 9 as well as the period (.) character.
///
/// # Arguments
///
/// * `it` - The passed number of our input.
fn take_number(it: &mut Peekable<Chars>) -> Token {
    let mut val = String::new();

    // We can not use take_while here, as it always consumes the next token
    // instead of just peeking it.
    while let Some(c) = it.peek() {
        if !c.is_numeric() && *c != '.' {
            break;
        }
        val.push(it.next().unwrap())
    }

    Token::Num(val.parse().unwrap())
}

/// Takes a string from the characters. Strings start and stop with a
/// quotation mark.
/// This function assumes the passed iterator to have the opening quotation
/// mark at the beginning and skips it without further checks.
///
/// # Arguments
///
/// * `it` - The passed string of our input.
fn take_str(it: &mut Peekable<Chars>) -> Token {
    // Skip the leading quotation mark without any further checks. This is
    // fine here, as we control all the invocations of this function.
    Token::Str(it.skip(1).take_while(|&c| c != '"').collect())
}

/// Takes an identifier or boolean from the characters. The token is assumed to
/// end at the first occurrence of whitespace.
///
/// # Arguments
///
/// * `it` - The passed identifier of our input.
fn take_ident_or_bool(it: &mut Peekable<Chars>) -> Token {
    let mut val = String::new();

    while let Some(c) = it.peek() {
        if c.is_whitespace() || *c == '(' || *c == ')' {
            break;
        }
        val.push(it.next().unwrap())
    }

    match val.as_str() {
        "true" | "false" => Token::Bool(val.parse().unwrap()),
        _ => Token::Ident(val),
    }
}

#[cfg(test)]
mod test {
    use super::Token::*;
    use super::*;

    macro_rules! test_tokenize {
        ($($name:ident: $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let out = tokenize($input);
                    assert_eq!(out, $expected);
                }
            )*
        };
    }

    test_tokenize!(
        tokenize_empty: "()" => vec![Paren('('), Paren(')')],
        tokenize_integer: "1" => vec![Num(1.0)],
        tokenize_long_integer: "1234" => vec![Num(1234.0)],
        tokenize_float: "1.234" => vec![Num(1.234)],
        tokenize_str: "\"foo\"" => vec![Str("foo".to_string())],
        tokenize_bool_true: "true" => vec![Bool(true)],
        tokenize_bool_false: "false" => vec![Bool(false)],
        tokenize_expr: "(foo 1 \"bar\" false 2)" => vec![
            Paren('('),
            Ident("foo".to_string()),
            Num(1.0),
            Str("bar".to_string()),
            Bool(false),
            Num(2.0),
            Paren(')')
        ],
        tokenize_bool_expr: "(= true false)" => vec![
            Paren('('),
            Ident("=".to_string()),
            Bool(true),
            Bool(false),
            Paren(')')
        ],
        tokenize_nested_if_expr: "(if (all true false) 1 2)" => vec![
            Paren('('),
            Ident("if".to_string()),
            Paren('('),
            Ident("all".to_string()),
            Bool(true),
            Bool(false),
            Paren(')'),
            Num(1.0),
            Num(2.0),
            Paren(')')
        ]
    );
}
