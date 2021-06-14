use thiserror::Error;

use crate::prelude::Prelude;
pub use crate::token::Token;

mod tokenize;
mod parse;
mod interpret;
mod prelude;
mod token;


/// Using an enum for Error Handling to call the right message when an error occurs and so easily
/// replacing the panic!() call.
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
    MissingTokens,
}

/// Type resulting either a success (`Ok`) or failure (`Err`)
pub type Result<T> = std::result::Result<T, Error>;


/// The name of our used Hashmap passed in a struct.
pub struct Interpreter {
    env: Prelude,
}

/// Implementing the Interpreter for our Hashmap.
/// get_prelude(): parsing the needed arguments and function for each identifier to HashMap.
/// Which is actually the initialization of our Map.
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { env: prelude::get_prelude() }
    }

    /// This function is the `heart` so that our Lisp Interpreter will work.
    /// We call each function, which handels each step, to get our final result and interpreter.
    /// # Arguments
    ///
    /// * `input` - Our input from the terminal
    ///
    pub fn interpret(&self, input: &str) -> Result<Token> {
        let token_stream = tokenize::tokenize(input);
        let expr = parse::parse(token_stream)?;
        let out = interpret::interpret(expr, &self.env)?;
        Ok(out)
    }
}

/// When the given arguments are wrong, instead of normally parse anything, we create our new
/// Interpreter -> Hash Map
impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}
