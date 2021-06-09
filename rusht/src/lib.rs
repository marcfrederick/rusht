use std::collections::HashMap;

use thiserror::Error;

pub use crate::tokenize::Token;

mod tokenize;
mod parse;
mod interpret;
mod prelude;


#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParserError(#[from] parse::Error),
}

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
