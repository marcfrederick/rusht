//! In prelude we define our hash map with its key (operator)
//! and the belonging value (called function with passed arguments).
//! Depending on the called operator we defined each a function.
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::stdin;

use crate::expr::Expr;
use crate::{Env, Error, Result};

/// Using macros to initialize the hash map in an easier and compact way.
/// Each entry of the map has a key and the belongig value.
/// The key presents an operator that maps to the needed function.
macro_rules! prelude {
    ($($key:expr => $val:expr),*) => {
        {
            let mut hash_map: Env = HashMap::new();
            $(
                hash_map.insert($key.to_string(), Expr::Func($val));
            )*
            hash_map
        }
    };
}

/// Returns a prelude (standard library) of often used functions.
pub fn get_prelude() -> Env {
    prelude!(
        "+" => |args| reduce(args, |a, b| -> f64 { a + b }),
        "-" => |args| reduce(args, |a, b| -> f64 { a - b }),
        "*" => |args| reduce(args, |a, b| -> f64 { a * b }),
        "/" => |args| reduce(args, |a, b| -> f64 { a / b }),
        "%" => |args| reduce(args, |a, b| -> f64 { a % b }),
        "concat" => |args| reduce(args, |a, b| -> String { format!("{}{}", a, b) }),
        "and" => |args| reduce(args, |a, b| -> bool { a && b }),
        "or" => |args| reduce(args, |a, b| -> bool { a || b }),
        "exit" => rusht_exit,
        "if" => rusht_if,
        "read" => rusht_read,
        "==" => rusht_strict_eq,
        "=" => |args| rusht_cmp(args, |a, b| (a - b).abs() < f64::EPSILON),
        "<" => |args| rusht_cmp(args, |a, b| a < b),
        "<=" => |args| rusht_cmp(args, |a, b| a <= b),
        ">" => |args| rusht_cmp(args, |a, b| a > b),
        ">=" => |args| rusht_cmp(args, |a, b| a >= b)
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
/// * `InvalidNumberOfArguments` - If there are too less or too many passed arguments.
fn rusht_if(args: Vec<Expr>) -> Result<Expr> {
    match args.as_slice() {
        [cond, on_true, on_false] => match cond.clone().try_into() {
            Ok(true) => Ok(on_true.clone()),
            Ok(false) => Ok(on_false.clone()),
            Err(err) => Err(err),
        },
        &_ => Err(Error::InvalidNumberOfArguments),
    }
}

/// Reads a line from the terminal.
///
/// # Arguments
///
/// * `_` - The upcoming input via terminal.
fn rusht_read(_: Vec<Expr>) -> Result<Expr> {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("failed to read from console");
    Ok(Expr::Str(buf))
}

/// Compares the given `args` strictly, meaning they must be of the same type
/// and value.
///
/// # Arguments
///
/// * `args` - The arguments passed to the function.
fn rusht_strict_eq(args: Vec<Expr>) -> Result<Expr> {
    Ok(Expr::Bool(args.windows(2).all(|w| w[0] == w[1])))
}

/// Compares the numeric values of its arguments using a given comparator
/// function. The comparison is performed loosely, meaning all values are
/// coerced to numbers before being compared.
///
/// # Arguments
///
/// * `args` - The arguments passed to the function.
/// * `cmp` - A comparison function taking two subsequent values.
///
/// # Errors
///
/// * `TypeError` - If one or more of the arguments can't be coerced to a
///     number.
fn rusht_cmp<F>(args: Vec<Expr>, cmp: F) -> Result<Expr>
where
    F: Fn(f64, f64) -> bool,
{
    Ok(args
        .into_iter()
        .map(Expr::try_into)
        .collect::<Result<Vec<f64>>>()?
        .windows(2)
        .all(|w| cmp(w[0], w[1]))
        .into())
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
fn rusht_exit(args: Vec<Expr>) -> Result<Expr> {
    let status_code = match args.as_slice() {
        [] => Ok(0.0),
        [status_code] => status_code.clone().try_into(),
        &_ => Err(Error::InvalidNumberOfArguments),
    }?;
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
/// `InvalidNumberOfArguments` - If the vector of args is empty, an error type will be returned.
///
/// # Panics
///
/// If one of the args can't be converted to a matching type, a panic occurs.
fn reduce<T, F>(args: Vec<Expr>, reducer: F) -> Result<Expr>
where
    T: TryFrom<Expr, Error = Error> + Into<Expr>,
    F: Fn(T, T) -> T,
{
    args.into_iter()
        .map(Expr::try_into)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .reduce(reducer)
        .ok_or(Error::InvalidNumberOfArguments)
        .map(T::into)
}

#[cfg(test)]
mod test {
    use super::Expr::*;
    use super::*;

    macro_rules! test_prelude {
        ($($name:ident => $key:expr; $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    match get_prelude().get($key).expect("function name not found in prelude") {
                        Expr::Func(func) => assert_eq!(func($input), $expected),
                        _ => panic!("expression is not a function")
                    }
                }
            )*
        };
    }

    test_prelude!(
        add_two => "+"; vec![Num(1.0), Num(2.0)] => Ok(Num(3.0)),
        add_three => "+"; vec![Num(1.0), Num(2.0), Num(2.0)] => Ok(Num(5.0)),
        add_with_corecion => "+"; vec![Bool(true), Str("5".to_string())] => Ok(Num(6.0)),
        sub => "-"; vec![Num(5.0), Num(2.0)] => Ok(Num(3.0)),
        mul => "*"; vec![Num(5.0), Num(2.0)] => Ok(Num(10.0)),
        div => "/"; vec![Num(5.0), Num(2.0)] => Ok(Num(2.5)),
        modul_num => "%"; vec![Num(1.0), Num(4.0)] => Ok(Num(1.0)),
        modul_bool => "%"; vec![Num(8.0), Bool(true)] => Ok(Num(0.0)),
        concat => "concat"; vec![Str("foo".to_string()), Str("bar".to_string())] => Ok(Str("foobar".to_string())),
        and_two => "and"; vec![Bool(true), Bool(true)] => Ok(Bool(true)),
        and_three => "and"; vec![Bool(true), Bool(false), Bool(true)] => Ok(Bool(false)),
        or_two => "or"; vec![Bool(false), Bool(false)] => Ok(Bool(false)),
        or_three => "or"; vec![Bool(true), Bool(false), Bool(true)] => Ok(Bool(true)),
        coercion_error => "-"; vec![Bool(true), Str("foo".to_string())] => Err(Error::CouldNotCoerceType),
        if_true => "if"; vec![Bool(true), Num(1.0), Num(2.0)] => Ok(Num(1.0)),
        if_false => "if"; vec![Bool(false), Num(1.0), Num(2.0)] => Ok(Num(2.0)),
        if_no_conditional => "if"; vec![Str("foo".to_string()), Num(1.0), Num(2.0)] => Err(Error::CouldNotCoerceType),
        if_too_few_args => "if"; vec![Bool(true), Num(1.0)] => Err(Error::InvalidNumberOfArguments),
        if_too_many_args => "if"; vec![Bool(true), Num(1.0), Num(2.0), Num(3.0)] => Err(Error::InvalidNumberOfArguments),
        bigger => ">"; vec![Num(10.0), Num(8.0)] => Ok(Bool(true)),
        equal_bigger => ">="; vec![Num(1.0), Num(1.0)] => Ok(Bool(true)),
        equal_bigger_bool => ">="; vec![Bool(false), Num(1.0)] => Ok(Bool(false)),
        smaller => "<"; vec![Num(5.0), Num(4.9)] => Ok(Bool(false)),
        equal_smaller => "<="; vec![Num(3.0), Num(3.1)] => Ok(Bool(true)),
        compare_true => "=="; vec![Num(4.0), Num(4.0)]=> Ok(Bool(true)),
        compare_false => "=="; vec![Num(4.0), Num(3.0)] => Ok(Bool(false))
    );
}
