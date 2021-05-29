use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Paren(char),
    Num(f64),
    Str(String),
    Ident(String),
    Bool(bool),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut it = input.chars().peekable();
    while let Some(c) = it.peek() {
        match c {
            '(' | ')' => tokens.push(Token::Paren(it.next().unwrap())),
            '0'..='9' => tokens.push(take_number(&mut it)),
            '"' => tokens.push(take_str(&mut it)),
            _ if c.is_whitespace() => { it.next(); }
            _ => tokens.push(take_ident_or_bool(&mut it))
        };
    }

    tokens
}

/// Takes a single number from the characters. Numbers are made up of the
/// numerals from 0 to 9 as well as the period (.) character.
fn take_number(it: &mut Peekable<Chars>) -> Token {
    let mut val = String::new();

    // We can not use take_while here, as it always consumes the next token
    // instead of just peeking it.
    while let Some(c) = it.peek() {
        if !c.is_numeric() && *c != '.' {
            break
        }
        val.push(it.next().unwrap())
    }

    Token::Num(val.parse().unwrap())
}

/// Takes a string from the characters. Strings start and stop with a
/// quotation mark.
/// This function assumes the passed iterator to have the opening quotation
/// mark at the beginning and skips it without further checks.
fn take_str(it: &mut Peekable<Chars>) -> Token {
    // Skip the leading quotation mark without any further checks. This is
    // fine here, as we control all the invocations of this function.
    Token::Str(it.skip(1)
        .take_while(|&c| c != '"')
        .collect())
}

/// Takes a identifier or boolean from the characters. The token is assumed to
/// end at the first occurrence of whitespace.
fn take_ident_or_bool(it: &mut Peekable<Chars>) -> Token {
    let mut val = String::new();

    while let Some(c) = it.peek() {
        if c.is_whitespace() {
            break
        }
        val.push(it.next().unwrap())
    }

    match val.as_str() {
        "true" | "false" => Token::Bool(val.parse().unwrap()),
        _ => Token::Ident(val)
    }
}

#[cfg(test)]
mod test {
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
        tokenize_empty: "()" => vec![Token::Paren('('), Token::Paren(')')],
        tokenize_integer: "1" => vec![Token::Num(1.0)],
        tokenize_long_integer: "1234" => vec![Token::Num(1234.0)],
        tokenize_float: "1.234" => vec![Token::Num(1.234)],
        tokenize_str: "\"foo\"" => vec![Token::Str("foo".to_string())],
        tokenize_bool_true: "true" => vec![Token::Bool(true)],
        tokenize_bool_false: "false" => vec![Token::Bool(false)],
        tokenize_expr: "(foo 1 \"bar\" false 2)" => vec![
            Token::Paren('('),
            Token::Ident("foo".to_string()),
            Token::Num(1.0),
            Token::Str("bar".to_string()),
            Token::Bool(false),
            Token::Num(2.0),
            Token::Paren(')')
        ]
    );
}
