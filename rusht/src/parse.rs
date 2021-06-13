/// This is our Lisp Interpreter's second step:
/// Here we pass our made TokenStream and put it into an AbstractTree.
/// This tree handles each calculation/execution by split it into the right Ast to go through
/// an execution in the right way and get a deterministic result in the end.

use std::collections::VecDeque;

use crate::Error;
use crate::Result;
use crate::token::Token;

/// Creating an enum with the two data types for our Tree.
/// * Atom: which identifies all given characters in our TokenStream.
/// * List: which lists all seperated Asts.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Atom(Token),
    List(Vec<Expr>),
}


/// Here we use a VecDequeue to have the opportunity to easily iterate over our given TokenStream.
///
/// # Arguments
///
/// * `tokenstream` - Our created Tokenstream from the Tokenizer.
///
pub fn parse(token_stream: Vec<Token>) -> Result<Expr> {
    parse_it(&mut VecDeque::from(token_stream))
}


/// Iterates over the given TokenStream and check the input to be able to create the Tree in the
/// correct and manageable way.
///
/// # Arguments
///
/// * `tokenstream` - Given expressions via a Queue.
///
/// # Errors
///
/// If the tokenstream surprisingly ends.
fn parse_it(token_stream: &mut VecDeque<Token>) -> Result<Expr> {
    let token = token_stream.pop_front()
        .ok_or(Error::UnexpectedEndOfTokenStream)?;

/// To create an Ast for each expression right, we check the braces that separate each execution.
///
/// # Errors
///
/// If the the expression's brace is missing or started with the closed one.
    match token {
        Token::Paren('(') => {
            let mut l = vec![];
            // TODO: Ugly code, clean this upµ⁄µ
            while *token_stream.get(0).ok_or(Error::MissingClosingParenthesis)? != Token::Paren(')') {
                l.push(parse_it(token_stream)?);
            }
            Ok(Expr::List(l))
        }
        Token::Paren(')') => Err(Error::UnexpectedClosingParenthesis),
        atom => Ok(Expr::Atom(atom))
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
        test_empty: vec![Paren('('), Paren(')')] => Ok(Expr::List(vec![])),
        test_single: vec![Paren('('), Num(4.0), Paren(')')] => Ok(Expr::List(vec![Expr::Atom(Num(4.0))])),
        test_nested: vec![Paren('('), Num(4.0), Paren('('), Num(5.0), Str("foo".to_string()), Paren(')'), Paren(')')] => Ok(Expr::List(vec![Expr::Atom(Num(4.0)), Expr::List(vec![Expr::Atom(Num(5.0)), Expr::Atom(Str("foo".to_string()))])])),
        test_unexpected_closing_paren: vec![Paren(')')] => Err(Error::UnexpectedClosingParenthesis),
        test_unclosed_expression: vec![Paren('(')] => Err(Error::MissingClosingParenthesis),
        test_unexpected_end_of_tokenstream: vec![] => Err(Error::UnexpectedEndOfTokenStream)
    );
}
