//! This is our Lisp Interpreter's third step:
//! Here we pass our built SyntaxTree.
//! If the tree is built up in the correct way, we can easily parse
//! through it and call the needed function with the passed arguments.
use std::convert::TryInto;

use crate::expr::{Expr, Lambda};
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
pub fn interpret(ast: Expr, env: &mut Env) -> Result<Expr> {
    match ast {
        expr @ (Expr::Bool(_) | Expr::Ident(_) | Expr::Str(_) | Expr::Num(_)) => Ok(expr),
        Expr::List(exprs) => match exprs.first() {
            Some(Expr::Ident(ident)) => match ident.as_str() {
                "def" => interpret_args(&exprs[1..], env).and_then(|args| rusht_def(&args, env)),
                "func" => rusht_lambda(&exprs[1..], env),
                _ => match env.get(ident).cloned() {
                    Some(Expr::Func(func)) => interpret_args(&exprs[1..], env).and_then(func),
                    Some(Expr::Lambda(lambda)) => interpret_lambda(lambda, &exprs[1..], env),
                    Some(_) => Err(Error::UnreadableTokens),
                    None => Err(Error::FunctionNotDefined(ident.to_string())),
                },
            },
            _ => Err(Error::UnreadableTokens),
        },
        _ => Err(Error::UnreadableTokens),
    }
}

fn interpret_lambda(lambda: Lambda, given_args: &[Expr], env: &Env) -> Result<Expr> {
    if lambda.args.len() != given_args.len() {
        return Err(Error::InvalidNumberOfArguments);
    }

    let mut local_env = env.clone();
    for (key, val) in lambda.args.iter().zip(&mut given_args.iter()) {
        local_env.insert(key.clone(), val.clone());
    }
    interpret(*lambda.body, &mut local_env)
}

/// Recursively interprets the arguments of the given slice of expressions.
///
/// # Arguments
///
/// * `args` - A slice of expressions to be interpreted.
/// * `env` - The global execution environment containing variable definitions.
///
/// # Errors
///
/// * `AttemptedToUseFunctionAsVariable` - When the arguments contain an
///     identifier that would resolve to a function definition.
/// * `VariableNotDefined` - When the arguments contain an identifier, for
///     which no corresponding value is found in the execution environment.
fn interpret_args(exprs: &[Expr], env: &mut Env) -> Result<Vec<Expr>> {
    exprs
        .iter()
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
fn resolve_variables(args: &[Expr], env: &mut Env) -> Result<Vec<Expr>> {
    args.iter()
        .map(|token| match token {
            Expr::Ident(var_name) => match env.get(var_name) {
                Some(Expr::Func(_)) => {
                    Err(Error::AttemptedToUseFunctionAsVariable(var_name.clone()))
                }
                Some(x) => Ok(x.clone()),
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
fn rusht_def(args: &[Expr], env: &mut Env) -> Result<Expr> {
    match args {
        [key, val] => {
            let key = key.clone().try_into()?;
            env.insert(key, val.clone());
            Ok(val.clone())
        }
        _ => Err(Error::InvalidNumberOfArguments),
    }
}

fn rusht_lambda(exprs: &[Expr], _env: &mut Env) -> Result<Expr> {
    match exprs {
        [Expr::List(args), body] if args.iter().all(|x| matches!(x, Expr::Ident(_))) => {
            let args = args
                .iter()
                .cloned()
                .map(|x| match x {
                    Expr::Ident(x) => Ok(x),
                    _ => Err(Error::CouldNotCoerceType),
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(Expr::Lambda(Lambda {
                args,
                body: Box::from(body.clone()),
            }))
        }
        [_, _] => Err(Error::CouldNotCoerceType),
        &_ => Err(Error::InvalidNumberOfArguments),
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
                Expr::Ident(String::from("+")),
                Expr::Num(4.0),
                Expr::Num(5.0),
                Expr::Num(15.0),
            ]),
            &mut prelude::get_prelude(),
        );
        assert_eq!(out, Ok(Expr::Num(24.0)))
    }

    #[test]
    fn nested_add() {
        let out = interpret(
            Expr::List(vec![
                Expr::Ident(String::from("+")),
                Expr::Num(4.0),
                Expr::Num(5.0),
                Expr::List(vec![
                    Expr::Ident(String::from("+")),
                    Expr::Num(10.0),
                    Expr::Num(5.0),
                ]),
            ]),
            &mut prelude::get_prelude(),
        );
        assert_eq!(out, Ok(Expr::Num(24.0)))
    }

    #[test]
    fn test_def() {
        let mut env = HashMap::new();

        interpret(
            Expr::List(vec![
                Expr::Ident("def".to_string()),
                Expr::Str("a".to_string()),
                Expr::Num(5.0),
            ]),
            &mut env,
        )
        .expect("error");

        assert_eq!(env.get("a").expect("key missing"), &Expr::Num(5.0))
    }

    #[test]
    fn test_def_and_use() {
        let mut env = get_prelude();

        interpret(
            Expr::List(vec![
                Expr::Ident("def".to_string()),
                Expr::Str("b".to_string()),
                Expr::Num(5.0),
            ]),
            &mut env,
        )
        .expect("error");

        let out = interpret(
            Expr::List(vec![
                Expr::Ident("+".to_string()),
                Expr::Ident("b".to_string()),
                Expr::Num(10.0),
            ]),
            &mut env,
        )
        .expect("error");

        assert_eq!(out, Expr::Num(15.0))
    }
}
