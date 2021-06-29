//! This is our Lisp Interpreter's third step:
//! Here we pass our built syntax tree.
//! If the tree is built up in the correct way, we can easily parse
//! through it and call the needed function with the passed arguments.
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
/// * `UnexpectedType` - If an unexpected type was encountered.
pub fn interpret(ast: Expr, env: &mut Env) -> Result<Expr> {
    match ast {
        expr @ (Expr::Bool(_) | Expr::Ident(_) | Expr::Str(_) | Expr::Num(_)) => Ok(expr),
        Expr::List(exprs) => match exprs.first() {
            Some(Expr::Ident(ident)) => match ident.as_str() {
                "def" => rusht_def(&exprs[1..], env),
                "func" => rusht_lambda(&exprs[1..]),
                "quote" => Ok(Expr::List(exprs[1..].to_vec())),
                _ => match env.get(ident).cloned() {
                    Some(Expr::Func(func)) => interpret_args(&exprs[1..], env).and_then(func),
                    Some(Expr::Lambda(lambda)) => interpret_lambda(lambda, &exprs[1..], env),
                    Some(_) => Err(Error::UnexpectedType),
                    None => Err(Error::FunctionNotDefined(ident.to_string())),
                },
            },
            Some(expr) => Err(Error::NotAnIdentifier(expr.to_string())),
            None => Err(Error::EmptyListExpression),
        },
        _ => Err(Error::UnexpectedType),
    }
}

/// Interprets a lambda expression and returns the resulting expression. A
/// lambda creates a copy of its surrounding execution environment.
///
/// # Arguments
///
/// * `lambda` - A lambda expression to be evaluated.
/// * `given_args` - The arguments passed at the invocation.
/// * `env` - The current execution environment.
fn interpret_lambda(lambda: Lambda, given_args: &[Expr], env: &Env) -> Result<Expr> {
    if lambda.args.len() != given_args.len() {
        return Err(Error::InvalidNumberOfArguments);
    }

    // create a local copy of the execution environment and add the passed
    // arguments as variables to this new local environment.
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
/// * `UnexpectedType` - If the first argument could not be coerced to a
///     string.
fn rusht_def(args: &[Expr], env: &mut Env) -> Result<Expr> {
    match args {
        [Expr::Ident(key), val] => {
            let val = interpret(val.clone(), env)?;
            env.insert(key.clone(), val.clone());
            Ok(val)
        }
        [_, _] => Err(Error::UnexpectedType),
        _ => Err(Error::InvalidNumberOfArguments),
    }
}

/// Constructs a lambda expression from the given arguments.
///
/// # Arguments
///
/// * `exprs[0]` - A list of identifiers representing the arguments of the
///     lambda expression.
/// * `exprs[1]` - The body of the lambda expression
///
/// # Errors
///
/// * `UnexpectedType` - If the first argument is not of type `Expr::List`.
/// * `InvalidNumberOfArguments` - If the number of arguments is not equal to
///     two.
fn rusht_lambda(exprs: &[Expr]) -> Result<Expr> {
    match exprs {
        [Expr::List(args), body] if args.iter().all(|x| matches!(x, Expr::Ident(_))) => {
            let args = args
                .iter()
                .cloned()
                .map(|x| match x {
                    Expr::Ident(x) => Ok(x),
                    _ => unreachable!("previously checked using the match guard"),
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(Expr::Lambda(Lambda {
                args,
                body: Box::from(body.clone()),
            }))
        }
        [_, _] => Err(Error::UnexpectedType),
        &_ => Err(Error::InvalidNumberOfArguments),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::prelude;
    use crate::prelude::create;

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
            &mut prelude::create(),
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
            &mut prelude::create(),
        );
        assert_eq!(out, Ok(Expr::Num(24.0)))
    }

    #[test]
    fn test_def() {
        let mut env = HashMap::new();

        interpret(
            Expr::List(vec![
                Expr::Ident("def".to_string()),
                Expr::Ident("a".to_string()),
                Expr::Num(5.0),
            ]),
            &mut env,
        )
        .expect("error");

        assert_eq!(env.get("a").expect("key missing"), &Expr::Num(5.0))
    }

    #[test]
    fn test_def_and_use() {
        let mut env = create();

        interpret(
            Expr::List(vec![
                Expr::Ident("def".to_string()),
                Expr::Ident("b".to_string()),
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

    #[test]
    fn test_lambda_hello() {
        let mut env = create();

        interpret(
            Expr::List(vec![
                Expr::Ident("def".to_string()),
                Expr::Ident("hello".to_string()),
                Expr::List(vec![
                    Expr::Ident("func".to_string()),
                    Expr::List(vec![Expr::Ident("name".to_string())]),
                    Expr::List(vec![
                        Expr::Ident("concat".to_string()),
                        Expr::Str("Hello, ".to_string()),
                        Expr::Ident("name".to_string()),
                        Expr::Str("!".to_string()),
                    ]),
                ]),
            ]),
            &mut env,
        )
        .expect("error");

        let out = interpret(
            Expr::List(vec![
                Expr::Ident("hello".to_string()),
                Expr::Str("Tester".to_string()),
            ]),
            &mut env,
        )
        .expect("error");

        assert_eq!(out, Expr::Str("Hello, Tester!".to_string()))
    }

    #[test]
    fn test_lambda_nums() {
        let mut env = create();

        interpret(
            Expr::List(vec![
                Expr::Ident("def".to_string()),
                Expr::Ident("adding".to_string()),
                Expr::List(vec![
                    Expr::Ident("func".to_string()),
                    Expr::List(vec![
                        Expr::Ident("a".to_string()),
                        Expr::Ident("b".to_string()),
                    ]),
                    Expr::List(vec![
                        Expr::Ident("+".to_string()),
                        Expr::Ident("a".to_string()),
                        Expr::Ident("b".to_string()),
                    ]),
                ]),
            ]),
            &mut env,
        )
        .expect("error");

        let out = interpret(
            Expr::List(vec![
                Expr::Ident("adding".to_string()),
                Expr::Num(3.0),
                Expr::Num(4.0),
            ]),
            &mut env,
        )
        .expect("error");

        assert_eq!(out, Expr::Num(7.0))
    }
}
