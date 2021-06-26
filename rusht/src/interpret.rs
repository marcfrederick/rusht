//! This is our Lisp Interpreter's third step:
//! Here we pass our built SyntaxTree.
//! If the tree is built up in the correct way, we can easily parse
//! through it and call the needed function with the passed arguments.
use std::convert::TryInto;

use crate::parse::Expr;
use crate::token::Token;
use crate::{Env, Error, Result};

/// Interprets the given abstract syntax tree, returning  either the resulting
/// token or an error.
///
/// * `ast` - An abstract syntax tree.
/// * `env` - The global execution environment containing variable definitions.
///
/// # Errors
///
/// * `AttemptedToUseFunctionAsVariable` - When the arguments contain an
///     identifier that would resolve to a function definition.
/// * `VariableNotDefined` - When the arguments contain an identifier, for
///     which no corresponding value is found in the execution environment.
/// * `FunctionNotDefined` - When attempting to call an undefined function.
/// * `UnreadableTokens` - TODO
pub fn interpret(ast: Expr, env: &mut Env) -> Result<Token> {
    match ast {
        Expr::Atom(token) => Ok(token),
        Expr::List(tokens) => match tokens.first() {
            Some(Expr::Atom(Token::Ident(ident))) => match ident.as_str() {
                "def" => interpret_args(&tokens, env).and_then(|args| rusht_def(&args, env)),
                "func" => todo!("function definition"),
                _ => match env.get(ident).cloned() {
                    Some(Expr::Func(func)) => interpret_args(&tokens, env).and_then(func),
                    Some(_) => Err(Error::UnreadableTokens),
                    None => Err(Error::FunctionNotDefined(ident.to_string())),
                },
            },
            _ => Err(Error::UnreadableTokens),
        },
        _ => Err(Error::UnreadableTokens),
    }
}

/// Recursively interprets the arguments of the given slice of expressions.
///
/// # Arguments
///
/// * `args` - A slice of expressions, from which to interpret the argument
///     part (everything from index 1 upwards)
/// * `env` - The global execution environment containing variable definitions.
///
/// # Errors
///
/// * `AttemptedToUseFunctionAsVariable` - When the arguments contain an
///     identifier that would resolve to a function definition.
/// * `VariableNotDefined` - When the arguments contain an identifier, for
///     which no corresponding value is found in the execution environment.
fn interpret_args(tokens: &[Expr], env: &mut Env) -> Result<Vec<Token>> {
    tokens
        .iter()
        .skip(1)
        .cloned()
        .map(|t| interpret(t, env))
        .collect::<Result<Vec<_>>>()
        .and_then(|args| resolve_variables(&args, env))
}

/// Replaces identifiers in the given slice of tokens with their corresponding
/// values from the environment.
///
/// # Arguments
///
/// * `args` - A slice of tokens, in which variables should be resolved.
/// * `env` - The global execution environment containing variable definitions.
///
/// # Errors
///
/// * `AttemptedToUseFunctionAsVariable` - When the arguments contain an
///     identifier that would resolve to a function definition.
/// * `VariableNotDefined` - When the arguments contain an identifier, for
///     which no corresponding value is found in the execution environment.
fn resolve_variables(args: &[Token], env: &mut Env) -> Result<Vec<Token>> {
    args.iter()
        .map(|token| match token {
            Token::Ident(var_name) => match env.get(var_name) {
                Some(Expr::Atom(x)) => Ok(x.clone()),
                Some(_) => Err(Error::AttemptedToUseFunctionAsVariable(var_name.clone())),
                None => Err(Error::VariableNotDefined(var_name.clone())),
            },
            x => Ok(x.clone()),
        })
        .collect::<Result<Vec<_>>>()
}

/// Defines or updates a variable in the environment.
///
/// # Arguments
///
/// * `args` - The arguments passed at the `def` function invocation. Should
///     have a length of exactly two elements, the variable name and value.
/// * `env` - The global execution environment containing the existing function
///     and variable definitions.
///
/// # Errors
///
/// * `InvalidNumberOfArguments` - If the length of `args` is not 2.
/// * `CouldNotCoerceType` - If the first argument could not be coerced to a
///     string.
fn rusht_def(args: &[Token], env: &mut Env) -> Result<Token> {
    match args {
        [key, val] => {
            let key = key.clone().try_into()?;
            env.insert(key, Expr::Atom(val.clone()));
            Ok(val.clone())
        }
        _ => Err(Error::InvalidNumberOfArguments),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::prelude;
    use crate::prelude::get_prelude;

    use super::*;

    #[test]
    fn single_add() {
        let out = interpret(
            Expr::List(vec![
                Expr::Atom(Token::Ident(String::from("+"))),
                Expr::Atom(Token::Num(4.0)),
                Expr::Atom(Token::Num(5.0)),
                Expr::Atom(Token::Num(15.0)),
            ]),
            &mut prelude::get_prelude(),
        );
        assert_eq!(out, Ok(Token::Num(24.0)))
    }

    #[test]
    fn nested_add() {
        let out = interpret(
            Expr::List(vec![
                Expr::Atom(Token::Ident(String::from("+"))),
                Expr::Atom(Token::Num(4.0)),
                Expr::Atom(Token::Num(5.0)),
                Expr::List(vec![
                    Expr::Atom(Token::Ident(String::from("+"))),
                    Expr::Atom(Token::Num(10.0)),
                    Expr::Atom(Token::Num(5.0)),
                ]),
            ]),
            &mut prelude::get_prelude(),
        );
        assert_eq!(out, Ok(Token::Num(24.0)))
    }

    #[test]
    fn test_def() {
        let mut env = HashMap::new();

        interpret(
            Expr::List(vec![
                Expr::Atom(Token::Ident("def".to_string())),
                Expr::Atom(Token::Str("a".to_string())),
                Expr::Atom(Token::Num(5.0)),
            ]),
            &mut env,
        )
        .expect("error");

        assert_eq!(
            env.get("a").expect("key missing"),
            &Expr::Atom(Token::Num(5.0))
        )
    }

    #[test]
    fn test_def_and_use() {
        let mut env = get_prelude();

        interpret(
            Expr::List(vec![
                Expr::Atom(Token::Ident("def".to_string())),
                Expr::Atom(Token::Str("b".to_string())),
                Expr::Atom(Token::Num(5.0)),
            ]),
            &mut env,
        )
        .expect("error");

        let out = interpret(
            Expr::List(vec![
                Expr::Atom(Token::Ident("+".to_string())),
                Expr::Atom(Token::Ident("b".to_string())),
                Expr::Atom(Token::Num(10.0)),
            ]),
            &mut env,
        )
        .expect("error");

        assert_eq!(out, Token::Num(15.0))
    }
}
