/// This is our Lisp Interpreter's third step:
/// Here we pass our built Tree.
/// If the tree is built up in the correct way, we can easily parse through it and call the using
/// function with passing the needed arguments.
/// So here we interpret out terminal input and do the executions.

use crate::{Env, Error, Result};
use crate::parse::Expr;
use crate::token::Token;

/// Interprets the given Types of the Token-Tree using the given `ast`.
/// Splitting the given ast into the final function and its passed arguments.
/// The result is putting everything into the Hash Map and calling the function.
///
/// # Arguments
///
/// * `ast` - Given expressions.
/// * `env` - Each expression will be passed to given Hash Map.
///
/// # Errors
///
/// If the vector of needed numbers/arguments for calculation/function is empty, an error type will
/// be returned.
pub fn interpret(ast: Expr, env: &mut Env) -> Result<Token> {
    match ast {
        Expr::Atom(token) => Ok(token),
        Expr::List(tokens) => {
            let tokens = tokens.iter()
                .map(|t| interpret(t.clone(), env))
                .collect::<Result<Vec<_>>>()?;

            let (func, args) = tokens.split_at(1);
            match func.get(0).ok_or(Error::UnreadableTokens)? {
                Token::Ident(ident) => {
                    let ident = (*ident).as_str();
                    let func = env.get(ident).ok_or_else(|| Error::FunctionNotDefined(ident.to_string()))?;
                    match func {
                        Expr::Func(func) => func(args.to_vec()),
                        _ => Err(Error::UnreadableTokens)
                    }
                }
                _ => Err(Error::UnreadableTokens)
            }
        }
        _ => Err(Error::UnreadableTokens)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude;

    use super::*;

    #[test]
    fn single_add() {
        let out = interpret(Expr::List(vec![
            Expr::Atom(Token::Ident(String::from("+"))),
            Expr::Atom(Token::Num(4.0)),
            Expr::Atom(Token::Num(5.0)),
            Expr::Atom(Token::Num(15.0)),
        ]), &mut prelude::get_prelude());
        assert_eq!(out, Ok(Token::Num(24.0)))
    }

    #[test]
    fn nested_add() {
        let out = interpret(Expr::List(vec![
            Expr::Atom(Token::Ident(String::from("+"))),
            Expr::Atom(Token::Num(4.0)),
            Expr::Atom(Token::Num(5.0)),
            Expr::List(vec![
                Expr::Atom(Token::Ident(String::from("+"))),
                Expr::Atom(Token::Num(10.0)),
                Expr::Atom(Token::Num(5.0)),
            ]),
        ]), &mut prelude::get_prelude());
        assert_eq!(out, Ok(Token::Num(24.0)))
    }
}
