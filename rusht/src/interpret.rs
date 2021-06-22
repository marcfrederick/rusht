/// This is our Lisp Interpreter's third step:
/// Here we pass our built Tree.
/// If the tree is built up in the correct way, we can easily parse through it and call the using
/// function with passing the needed arguments.
/// So here we interpret out terminal input and do the executions.

use crate::{Env, Error, Result};
use crate::parse::Expr;
use crate::token::Token;
use std::convert::TryInto;

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
            match func.get(0) {
                Some(Token::Ident(ident)) => {
                    let args = resolve_variables(env, args)?;
                    match ident.as_str() {
                        "def" => rusht_def(args, env),
                        _ => {
                            match env.get(ident) {
                                Some(Expr::Func(func)) => func(args),
                                Some(_) => Err(Error::UnreadableTokens),
                                None => Err(Error::FunctionNotDefined(ident.to_string())),
                            }
                        }
                    }
                }
                Some(_) => Err(Error::UnreadableTokens),
                None => Err(Error::UnreadableTokens),
            }
        }
        _ => Err(Error::UnreadableTokens)
    }
}

fn resolve_variables(env: &mut Env, args: &[Token]) -> Result<Vec<Token>> {
    Ok(args.iter()
        .map(|token| match token {
            Token::Ident(var_name) => match env.get(var_name) {
                Some(Expr::Atom(x)) => Ok(x),
                Some(_) => Err(Error::AttemptedToUseFunctionAsVariable(var_name.clone())),
                None => Err(Error::VariableNotDefined(var_name.clone()))
            },
            x => Ok(x),
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .cloned()
        .collect::<Vec<_>>())
}

/// Defines a variable in the given environment.
fn rusht_def(args: Vec<Token>, env: &mut Env) -> Result<Token> {
    if args.len() != 2 {
        return Err(Error::InvalidNumberOfArguments);
    }

    // We can safely unwrap here as we've previously validated the number of
    // arguments.
    let key = args.get(0).unwrap().clone().try_into()?;
    let val = args.get(1).unwrap();
    env.insert(key, Expr::Atom(val.clone()));

    Ok(val.clone())
}

#[cfg(test)]
mod test {
    use crate::prelude;

    use super::*;
    use std::collections::HashMap;
    use crate::prelude::get_prelude;

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

    #[test]
    fn test_def() {
        let mut env = HashMap::new();

        interpret(Expr::List(vec![
            Expr::Atom(Token::Ident("def".to_string())),
            Expr::Atom(Token::Str("a".to_string())),
            Expr::Atom(Token::Num(5.0)),
        ]), &mut env).expect("error");

        assert_eq!(env.get("a").expect("key missing"), &Expr::Atom(Token::Num(5.0)))
    }

    #[test]
    fn test_def_and_use() {
        let mut env = get_prelude();

        interpret(Expr::List(vec![
            Expr::Atom(Token::Ident("def".to_string())),
            Expr::Atom(Token::Str("b".to_string())),
            Expr::Atom(Token::Num(5.0)),
        ]), &mut env).expect("error");

        let out = interpret(Expr::List(vec![
            Expr::Atom(Token::Ident("+".to_string())),
            Expr::Atom(Token::Ident("b".to_string())),
            Expr::Atom(Token::Num(10.0)),
        ]), &mut env).expect("error");

        assert_eq!(out, Token::Num(15.0))
    }
}
