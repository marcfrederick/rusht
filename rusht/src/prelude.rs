use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::stdin;

use crate::{Error, Result};
use crate::token::Token;
use thiserror::private::DisplayAsDisplay;

/// Using macros to initialize the hash map in an easier and compact way.
macro_rules! prelude {
    ($($key:expr => $val:expr),*) => {
        {
            let mut hash_map: Prelude = HashMap::new();
            $(
                hash_map.insert($key.to_string(), $val);
            )*
            hash_map
        }
    };
}

/// A key value mapping of function names and the accompanying implementation.
pub type Prelude = HashMap<String, fn(Vec<Token>) -> Result<Token>>;

/// Returns a prelude (standard library) of often used functions.
pub fn get_prelude() -> Prelude {
    prelude!(
        "+" => |args| reduce(args, |a, b| -> f64 { a + b }),
        "-" => |args| reduce(args, |a, b| -> f64 { a - b }),
        "*" => |args| reduce(args, |a, b| -> f64 { a * b }),
        "/" => |args| reduce(args, |a, b| -> f64 { a / b }),
        "concat" => |args| reduce(args, |a, b| -> String { format!("{}{}", a, b) }),
        "and" => |args| reduce(args, |a, b| -> bool { a && b }),
        "or" => |args| reduce(args, |a, b| -> bool { a || b }),
        "exit" => rusht_exit,
        "if" => rusht_if,
        "read" => rusht_read,
        "def" => rusht_varialbe_define
    )
}

/// Checks a given condition and returns one of two possible values.
///
/// # Arguments
///
/// * `args[0]` - A condition to be checked.
/// * `args[1]` - The value to be returned if the condition is truthy.
/// * `args[2]` - The value to be returned if the condition is not truthy.
///
/// # Errors
///
/// * `TypeError` - If the given condition can't be coerced to a bool.
fn rusht_if(args: Vec<Token>) -> Result<Token> {
    if args.len() != 3 {
        return Err(Error::InvalidNumberOfArguments);
    }

    // We can safely use unwrap here, as we've previously checked the number of
    // arguments given to the function.
    let condition = args.get(0).unwrap().clone().try_into()?;
    let out_index = if condition { 1 } else { 2 };
    Ok(args.get(out_index).unwrap().clone())
}


fn variable_declare(args: Vec<Token>) -> Result<Token> {
    if args.len() != 2 {
        return Err(Error::InvalidNumberOfArguments);
    }
    let num = args.get(1).unwrap().clone();
    let value = args.get(0).insert(&num).clone();
    Ok(Token::Num(num))
}


/// Reads a line from the console.
fn rusht_read(_: Vec<Token>) -> Result<Token> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("failed to read from console");
    Ok(Token::Str(buf))
}

/// Exits the current process with a given exit code or `0`.
///
/// # Arguments
///
/// * `args` - A consisting of either 0 or 1 elements.
///
/// # Errors
///
/// * `InvalidNumberOfArguments` - If the vector of args has a size greater
///     than 1.
/// * `TypeError` - If the given status code can't be coerced to a number.
fn rusht_exit(args: Vec<Token>) -> Result<Token> {
    if args.len() > 1 {
        return Err(Error::InvalidNumberOfArguments);
    }

    let status_code = args.get(0)
        .map(|token| token.clone().try_into())
        .unwrap_or(Ok(0.0))?;
    std::process::exit(status_code as i32);
}

/// Reduces the given vector of `Token`s  using the given `reducer` function.
///
/// # Arguments
///
/// * `args` - The arguments passed to the function.
/// * `reducer` - A function used to reduce the args to a single value.
///
/// # Errors
///
/// If the vector of args is empty, an error type will be returned.
///
/// # Panics
///
/// If one of the args can't be converted to a matching type, a panic occurs.
fn reduce<T, F>(args: Vec<Token>, reducer: F) -> Result<Token>
    where
        T: TryFrom<Token, Error=Error> + Into<Token>,
        F: Fn(T, T) -> T,
{
    args
        .into_iter()
        .map(|x| x.try_into())
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .reduce(reducer)
        .ok_or(Error::InvalidNumberOfArguments)
        .map(|x| x.into())
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Token::*;

    macro_rules! test_prelude {
        ($($name:ident => $key:expr; $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let prelude = get_prelude();
                    let actual = prelude.get($key).unwrap()($input);
                    assert_eq!(actual, $expected);
                }
            )*
        };
    }

    test_prelude!(
        add_two => "+"; vec![Num(1.0), Num(2.0)] => Ok(Num(3.0)),
        add_three => "add"; vec![Num(1.0), Num(2.0), Num(2.0)] => Ok(Num(5.0)),
        add_with_corecion => "+"; vec![Bool(true), Str("5".to_string())] => Ok(Num(6.0)),
        sub => "-"; vec![Num(5.0), Num(2.0)] => Ok(Num(3.0)),
        mul => "*"; vec![Num(5.0), Num(2.0)] => Ok(Num(10.0)),
        div => "/"; vec![Num(5.0), Num(2.0)] => Ok(Num(2.5)),
        concat => "concat"; vec![Str("foo".to_string()), Str("bar".to_string())] => Ok(Str("foobar".to_string())),
        and_two => "and"; vec![Bool(true), Bool(true)] => Ok(Bool(true)),
        and_three => "and"; vec![Bool(true), Bool(false), Bool(true)] => Ok(Bool(false)),
        or_two => "or"; vec![Bool(false), Bool(false)] => Ok(Bool(false)),
        or_three => "or"; vec![Bool(true), Bool(false), Bool(true)] => Ok(Bool(true)),
        coercion_error => "sub"; vec![Bool(true), Str("foo".to_string())] => Err(Error::CouldNotCoerceType),
        if_true => "if"; vec![Bool(true), Num(1.0), Num(2.0)] => Ok(Num(1.0)),
        if_false => "if"; vec![Bool(false), Num(1.0), Num(2.0)] => Ok(Num(2.0)),
        if_no_conditional => "if"; vec![Str("foo".to_string()), Num(1.0), Num(2.0)] => Err(Error::CouldNotCoerceType),
        if_too_few_args => "if"; vec![Bool(true), Num(1.0)] => Err(Error::InvalidNumberOfArguments),
        if_too_many_args => "if"; vec![Bool(true), Num(1.0), Num(2.0), Num(3.0)] => Err(Error::InvalidNumberOfArguments)
    );
}
