/// This is our Lisp Interpreter's second step:
/// Here we pass our TokenStream and put it into an AbstractTree.
/// The input gets splitted by going through the tokenstream and
/// split each stream's list correctly by parsing it to one knot
/// with the inside order to manage the right final execution.

use std::iter::Peekable;

use crate::Error;
use crate::Result;
use crate::token::Token;

/// Represents a lisp expression.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A single (i.e. atomic) value.
    Atom(Token),
    /// A list expressions.
    List(Vec<Expr>),
    /// A function type.
    Func(fn(Vec<Token>) -> Result<Token>),
}


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
        T: IntoIterator<Item=Token>
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
        T: Iterator<Item=Token>
{
    match token_stream.next().ok_or(Error::UnexpectedEndOfTokenStream)? {
        Token::Paren('(') => parse_nested_expression(token_stream),
        Token::Paren(')') => Err(Error::UnexpectedClosingParenthesis),
        atom => Ok(Expr::Atom(atom))
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
        T: Iterator<Item=Token>
{
    let mut list = vec![];
    while *token_stream.peek().ok_or(Error::MissingClosingParenthesis)? != Token::Paren(')') {
        list.push(parse_it(token_stream)?);
    }
    token_stream.next();
    Ok(Expr::List(list))
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
                    assert_eq!(parse($input), $expected);
                }
            )*
        };
    }

    test_parse!(
        test_empty: vec![Paren('('), Paren(')')] => Ok(Expr::List(vec![])),
        test_single: vec![Paren('('), Num(4.0), Paren(')')] => Ok(Expr::List(vec![Expr::Atom(Num(4.0))])),
        test_nested: vec![
            Paren('('),
            Num(4.0),
            Paren('('),
            Num(5.0),
            Str("foo".to_string()),
            Paren(')'),
            Paren(')')
        ] => Ok(Expr::List(vec![
            Expr::Atom(Num(4.0)),
            Expr::List(vec![
                Expr::Atom(Num(5.0)),
                Expr::Atom(Str("foo".to_string()))
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
            Expr::Atom(Ident("if".to_string())),
            Expr::Atom(Bool(true)),
            Expr::Atom(Num(2.0)),
            Expr::Atom(Num(4.0))
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
            Expr::Atom(Ident("if".to_string())),
            Expr::List(vec![
                Expr::Atom(Ident("and".to_string())),
                Expr::Atom(Bool(true)),
                Expr::Atom(Bool(true))
            ]),
            Expr::Atom(Num(2.0)),
            Expr::Atom(Num(4.0))
        ])),
        test_unexpected_closing_paren: vec![Paren(')')] => Err(Error::UnexpectedClosingParenthesis),
        test_unclosed_expression: vec![Paren('(')] => Err(Error::MissingClosingParenthesis),
        test_unexpected_end_of_tokenstream: vec![] => Err(Error::UnexpectedEndOfTokenStream)
    );
}
