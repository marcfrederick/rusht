//! This is our Lisp Interpreter's second step:
//! Here we pass our token stream and put it into an abstract syntax tree.
//! The input gets splitted by going through the tokenstream and
//! split each stream's list correctly by parsing it to one knot
//! with the inside order to manage the right final execution.
use std::convert::TryInto;
use std::iter::Peekable;

use crate::expr::Expr;
use crate::tokenize::Token;
use crate::Error;
use crate::Result;

/// Creates an abstract syntax tree from the given (non-empty) token stream.
/// Here we iterate throught the tokenstream and call
///
/// # Arguments
///
/// * `token_stream` - A vector containing the tokens to be parsed.
///
/// # Errors
///
/// * `UnexpectedEndOfTokenStream` - If the given token stream is empty.
/// * `MissingClosingParenthesis` - If the number of opening braces exceeds the
///     number of closing braces.
/// * `UnexpectedClosingParenthesis` - If the number of closing braces exceeds
///     the number of opening braces.
pub fn parse<T>(token_stream: T) -> Result<Expr>
where
    T: IntoIterator<Item = Token>,
{
    parse_it(&mut token_stream.into_iter().peekable())
}

/// Creates an abstract syntax tree from the given iterator of tokens.
/// If the braces in the token stream are not balanced, an error is returned.
///
/// # Arguments
///
/// * `token_stream` - A peekable iterator, containing the tokens to be parsed.
///
/// # Errors
///
/// * `UnexpectedEndOfTokenStream` - If the given token stream is empty.
/// * `MissingClosingParenthesis` - If the number of opening braces exceeds the
///     number of closing braces.
/// * `UnexpectedClosingParenthesis` - If the number of closing braces exceeds
///     the number of opening braces.
fn parse_it<T>(token_stream: &mut Peekable<T>) -> Result<Expr>
where
    T: Iterator<Item = Token>,
{
    match token_stream
        .next()
        .ok_or(Error::UnexpectedEndOfTokenStream)?
    {
        Token::Paren('(') => parse_nested_expression(token_stream),
        Token::Paren(')') => Err(Error::UnexpectedClosingParenthesis),
        atom => atom.try_into(),
    }
}

/// Parses a nested expression from the given token stream.
///
/// An expression begins at each opening brace and ends at the matching closing
/// brace.
///
/// # Arguments
///
/// * `token_stream` - A peekable iterator, containing the tokens to be parsed.
///
/// # Errors
///
/// * `MissingClosingParenthesis` - If the number of opening braces exceeds the
///     number of closing braces.
#[inline]
fn parse_nested_expression<T>(token_stream: &mut Peekable<T>) -> Result<Expr>
where
    T: Iterator<Item = Token>,
{
    let mut list = vec![];
    while *token_stream
        .peek()
        .ok_or(Error::MissingClosingParenthesis)?
        != Token::Paren(')')
    {
        list.push(parse_it(token_stream)?);
    }
    token_stream.next();
    Ok(Expr::List(list))
}

#[cfg(test)]
mod test {
    use super::Token::*;
    use super::*;

    macro_rules! test_parse {
        ($($name:ident: $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(parse($input), $expected);
                }
            )*
        };
    }

    test_parse!(
        test_empty: vec![Paren('('), Paren(')')] => Ok(Expr::List(vec![])),
        test_single: vec![Paren('('), Num(4.0), Paren(')')] => Ok(Expr::List(vec![Expr::Num(4.0)])),
        test_nested: vec![
            Paren('('),
            Num(4.0),
            Paren('('),
            Num(5.0),
            Str("foo".to_string()),
            Paren(')'),
            Paren(')')
        ] => Ok(Expr::List(vec![
            Expr::Num(4.0),
            Expr::List(vec![
                Expr::Num(5.0),
                Expr::Str("foo".to_string())
            ])
        ])),
        test_if: vec![
            Paren('('),
            Ident("if".to_string()),
            Bool(true),
            Num(2.0),
            Num(4.0),
            Paren(')')
        ] => Ok(Expr::List(vec![
            Expr::Ident("if".to_string()),
            Expr::Bool(true),
            Expr::Num(2.0),
            Expr::Num(4.0)
        ])),
        test_if_nested: vec![
            Paren('('),
            Ident("if".to_string()),
            Paren('('),
            Ident("and".to_string()),
            Bool(true),
            Bool(true),
            Paren(')'),
            Num(2.0),
            Num(4.0),
            Paren(')')
        ] => Ok(Expr::List(vec![
            Expr::Ident("if".to_string()),
            Expr::List(vec![
                Expr::Ident("and".to_string()),
                Expr::Bool(true),
                Expr::Bool(true)
            ]),
            Expr::Num(2.0),
            Expr::Num(4.0)
        ])),
        test_unexpected_closing_paren: vec![Paren(')')] => Err(Error::UnexpectedClosingParenthesis),
        test_unclosed_expression: vec![Paren('(')] => Err(Error::MissingClosingParenthesis),
        test_unexpected_end_of_tokenstream: vec![] => Err(Error::UnexpectedEndOfTokenStream)
    );
}
