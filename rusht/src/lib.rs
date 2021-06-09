use std::collections::HashMap;

use crate::tokenize::Token;

pub mod tokenize;
pub mod parse;
pub mod interpret;
pub mod prelude;

pub type Env = HashMap<String, fn(Vec<Token>) -> Token>;
