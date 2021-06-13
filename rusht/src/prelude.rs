use std::collections::HashMap;
// use std::process;

use crate::{Error, Result};
use crate::tokenize::Token;


/// Using macros to initialize the hash map in an easier and compact way.
macro_rules! prelude {
    ($($key:expr => $val:expr),*) => {
        {
            let mut hash_map: HashMap<String, fn(Vec<Token>) -> Token> = HashMap::new();
            $(
                hash_map.insert($key.to_string(), $val);
            )*
            hash_map
        }
    };
}


/// Using macros to pass the needed arguments, the calling calculation/execution and its right Token
/// Type of the called function.
macro_rules! reduce {
    ($reducer:expr => $finalizer:expr) => {
        |args| reduce(args, $reducer, $finalizer).unwrap()
    };
}

/*
macro_rules! exitprogram {
    ($exiter:expr => $final:expr) => {
        reduce($exiter, $final).unwrap()
    };
}
 */


/// Checking the passed `token` type - here f64.
/// And upgraded this type to use also String and Bool for calculation.
/// /// To be able to have more and handle options with terminal input.
impl From<Token> for f64 {
    fn from(token: Token) -> Self {
        match token {
            Token::Num(n) => n,
            Token::Bool(true) => 1.0,
            Token::Bool(false) => 0.0,
            Token::Str(s) if s.parse::<f64>().is_ok() => s.parse().unwrap(),
            _ => panic!()
        }
    }
}

/// Checking the passed `token` type - here String.
/// And upgraded this type to use also Num and Bool for strings.
/// To be able to have more and handle options with terminal input.
impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token {
            Token::Str(s) => s,
            Token::Bool(b) => b.to_string(),
            Token::Num(n) => n.to_string(),
            _ => panic!()
        }
    }
}

/// Checking the passed `token` type - here Bool.
impl From<Token> for bool {
    fn from(token: Token) -> Self {
        match token {
            Token::Bool(b) => b,
            Token::Num(x) if x == 0.0 => false,
            Token::Num(_) => true,
            Token::Str(s) if ["true", "1"].contains(&s.as_str()) => true,
            Token::Str(s) if ["false", "0", ""].contains(&s.as_str()) => false,
            _ => panic!()
        }
    }
}

/*
pub fn exit_func() -> ! {
    process::exit(1);
}

 */

/// A key value mapping of function names and the accompanying implementation.
pub type Prelude = HashMap<String, fn(Vec<Token>) -> Token>;

/// Returns a prelude (standard library) of often used functions.
pub fn get_prelude() -> Prelude {
    prelude!(
        "+" => reduce!(|a, b| a + b => Token::Num),
        "add" => reduce!(|a, b| a + b => Token::Num),
        "-" => reduce!(|a, b| a - b => Token::Num),
        "sub" => reduce!(|a, b| a - b => Token::Num),
        "*" => reduce!(|a, b| a * b => Token::Num),
        "mul" => reduce!(|a, b| a * b => Token::Num),
        "/" => reduce!(|a, b| a / b => Token::Num),
        "div" => reduce!(|a, b| a / b => Token::Num),
        "concat" => reduce!(|a, b| format!("{}{}", a, b) => Token::Str)
        //"exit()" => exitprogram!(exit_func => Token::Str)
    )
}

/// Reduces the given vector of `Token`s  using the given `reducer` function.
/// The result is turned back into a vector using the `finalizer` function.
///
/// # Arguments
///
/// * `args` - The arguments passed to the function.
/// * `reducer` - A function used to reduce the args to a single value.
/// * `finalizer` - A function used to turn the result back into a `Token`.
///
/// # Errors
///
/// If the vector of args is empty, an error type will be returned.
///
/// # Panics
///
/// If one of the args can't be converted to a matching type, a panic occurs.
fn reduce<T, F, G>(args: Vec<Token>, reducer: F, finalizer: G) -> Result<Token>
    where
        T: From<Token>,
        F: Fn(T, T) -> T,
        G: Fn(T) -> Token
{
    // The `finalizer` could be replaced by adding `+ Into<Token>` to the type
    // constraints of `T`. We have decided against this approach for the
    // added readability.
    // The alternative implementation would require us to specify the type
    // encapsulated in the specific `Token` variant, so not much writing effort
    // would be saved anyways.
    args
        .into_iter()
        .map(|x| x.into())
        .reduce(reducer)
        .ok_or(Error::InvalidNumberOfArguments)
        .map(finalizer)
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
        add_two => "+"; vec![Num(1.0), Num(2.0)] => Num(3.0),
        add_three => "add"; vec![Num(1.0), Num(2.0), Num(2.0)] => Num(5.0),
        sub => "-"; vec![Num(5.0), Num(2.0)] => Num(3.0),
        mul => "*"; vec![Num(5.0), Num(2.0)] => Num(10.0),
        div => "/"; vec![Num(5.0), Num(2.0)] => Num(2.5),
        concat => "concat"; vec![Str("foo".to_string()), Str("bar".to_string())] => Str("foobar".to_string()),
        and_two => "and"; vec![Bool(true), Bool(true)] => Bool(true),
        and_three => "and"; vec![Bool(true), Bool(false), Bool(true)] => Bool(false),
        or_two => "or"; vec![Bool(false), Bool(false)] => Bool(false),
        or_three => "or"; vec![Bool(true), Bool(false), Bool(true)] => Bool(true)
    );
}