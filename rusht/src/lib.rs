use std::collections::HashMap;

use thiserror::Error;

pub use crate::tokenize::Token;

mod tokenize;
mod parse;
mod interpret;
mod prelude;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("token stream ended unexpectedly")]
    UnexpectedEndOfTokenStream,
    #[error("encountered an unexpected closing parenthesis")]
    UnexpectedClosingParenthesis,
    #[error("missing expected closing parenthesis")]
    MissingClosingParenthesis,
    #[error("error")]
    TypeError,
    #[error("invalid number of arguments passed")]
    InvalidNumberOfArguments,
}

/// Type resulting either a success (`Ok`) or failure (`Err`)
pub type Result<T> = std::result::Result<T, Error>;

type Env = HashMap<String, fn(Vec<Token>) -> Token>;

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { env: prelude::get_prelude() }
    }

    pub fn interpret(&self, input: &str) -> Result<Token> {
        let token_stream = tokenize::tokenize(input);
        let expr = parse::parse(token_stream)?;
        let out = interpret::interpret(expr, &self.env);
        Ok(out)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}
