// impl TokenStream

use std::iter::Peekable;
use std::str::Chars;

use peeking_take_while::PeekableExt;

#[derive(Debug)]
pub enum Token {
    Atom(String), // Symbole für Mathematik
    Number(f64), // Zahlen
    Brace(char), // Klammern
}

pub fn math(sign: &mut Peekable<Chars>) -> f64
{
    let mut iter = sign.chars();

    match iter
    {
        '+' => add(),
        '-' => sub(),
        '*' => mult(),
        '/' => sub(),
    }
}

fn take_number(it: &mut Peekable<Chars>) -> f64 {
    it.peeking_take_while(|&c| ('0'..='9').contains(&c) || c == '.')
        .collect::<String>()
        .parse::<f64>()
        .unwrap()
}


pub fn tokenizer(input: &mut String) -> Vec<String>
{
    let tokenstream: Vec<String> = Vec::new();

    let mut iter = input.chars().peekable();
    while let Some(&next) = iter.peek() {
        match it 
        {
            '(' | ')' => tokenstream.push(Self::Brace),
            '0'..='9' => tokenstream.push(Self::Number(it.next().unwrap())),
            '+' | '-' | '*' | '/' => tokenstream.push(Self::Atom(it.next().unwrap())),
        }
}


// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://doc.rust-lang.org/rust-by-example/macros.html