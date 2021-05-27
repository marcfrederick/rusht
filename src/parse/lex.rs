// An iterator with a peek() that returns
// an optional reference to the next element
// Peekable is a single, specific, concrete type...
use std::iter::Peekable;
use std::str::Chars;

use peeking_take_while::PeekableExt;

// PartialEq -> eq for comparing
// Derive -> automatically creates the implementation
// required to make this `enum` printable
#[derive(Debug, PartialEq)]
pub enum Token {
    Atom(String), // Symbole für Mathematik
    Number(f64), // Zahlen
    Brace(char), // Klammern
}

fn take_number(it: &mut Peekable<Chars>) -> f64 {
    it.peeking_take_while(|&c| ('0'..='9').contains(&c) || c == '.')
        .collect::<String>()
        .parse::<f64>()
        .unwrap() //  will give you the embedded T
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = vec![];

    // https://stackoverflow.com/questions/26333439/how-to-use-rusts-peekable
    // ... that is constructed by calling peekable() on an interator
    let mut it = input.chars().peekable();
    while let Some(&next) = it.peek() { // peek(value)
        match next {
            '0'..='9' => tokens.push(Token::Number(take_number(&mut it))),
            '(' | ')' => tokens.push(Token::Brace(it.next().unwrap())),
            '+' | '-' | '/' | '*' => tokens.push(Token::Atom(it.next().unwrap())),
            _ => unimplemented!()
        };
    }

    tokens
}