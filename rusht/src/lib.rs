//! Lib is like our library.
//! Here we define our needed error for handling the
//! normal panic! calls.
//! And define our important used map to even be
//! able to handle the written identifiers which
//! are our operaters with the allocated execution.
use std::collections::HashMap;

use thiserror::Error;

pub use crate::expr::Expr;
pub use crate::tokenize::Token;

mod expr;
mod interpret;
mod parse;
mod prelude;
mod tokenize;

/// Using an enum for Error Handling to call the right message
/// when an error occurs.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("token stream ended unexpectedly")]
    UnexpectedEndOfTokenStream,
    #[error("encountered an unexpected closing parenthesis")]
    UnexpectedClosingParenthesis,
    #[error("missing expected closing parenthesis")]
    MissingClosingParenthesis,
    #[error("unable to coerce to correct type")]
    CouldNotCoerceType,
    #[error("invalid number of arguments passed")]
    InvalidNumberOfArguments,
    #[error("function `{0}` is not defined")]
    FunctionNotDefined(String),
    #[error("missing tokens or cannot be read")]
    UnreadableTokens,
    #[error("missing tokens for execution")]
    MissingTokens, //doesn't exist yet
    #[error("attempted to use function `{0}` as a variable")]
    AttemptedToUseFunctionAsVariable(String),
    #[error("variable `{0}` is not defined")]
    VariableNotDefined(String),
}

/// Type resulting either a success (`Ok`) or failure (`Err`)
pub type Result<T> = std::result::Result<T, Error>;

type Env = HashMap<String, Expr>;

/// The name of our used Hashmap passed in a struct.
#[derive(Debug, Default)]
pub struct Interpreter {
    env: Env,
}

/// Implementing the Interpreter for our Hashmap by parsing the
/// needed arguments and function for each identifier to HashMap
/// which is actually the initialization of our Map.
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: prelude::get_prelude(),
        }
    }

    /// This function is the heart so that our Lisp Interpreter will work.
    /// This function summarizes our three steps:
    /// the tokenstream which presents our input with the datatypes,
    /// the expression which presents our parser which handles the AbstractTree,
    /// and the out which presents our interpretation for the execution.
    ///
    /// * `input` - Our input from the terminal.
    ///
    pub fn interpret<T>(&mut self, input: T) -> Result<Expr>
    where
        T: AsRef<str>,
    {
        let token_stream = tokenize::tokenize(input.as_ref());
        let expr = parse::parse(token_stream)?;
        let out = interpret::interpret(expr, &mut self.env)?;
        Ok(out)
    }
}
