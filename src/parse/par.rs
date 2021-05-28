mod lex;


use std::iter::Peekable;
use std::str::Chars;

use peeking_take_while::PeekableExt;

#[derive(Debug)]
pub enum Token {
    Atom(String), // Symbole für Mathematik
    Number(f64), // Zahlen
    Brace(char), // Klammern
}


// Vector for checking the input
let stringstream: Vec<String> = Vec::new();


// Nr. 2
pub fn take_input(input: &mut [String]) -> Vec<String>
{
    let mut iter = input.chars().peekable();
    while let Some(&next) = iter.peek() {
        match it 
        {
            '(' | ')' => stringstream.push(Self::Brace),
            '0'..='9' => stringstream.push(Self::Number(it.next().unwrap())),
            '+' | '-' | '*' | '/' => stringstream.push(Self::Atom(it.next().unwrap())),
        }
}


fn take_number(num: &mut Peekable<Chars>) -> f64 {
    num.peeking_take_while(|&c| ('0'..='9').contains(&c) || c == '.')
        .collect::<String>()
        .parse::<f64>()
        .unwrap()
}


fn take_symbol(num: &mut Peekable<Chars>) -> String 
{
    let mut iter = input.chars().peekable();
    while let Some(&next) = iter.peek() {
        match it 
        {
            '(' | ')' => stringstream.push(Self::Brace),
            '+' | '-' | '*' | '/' => stringstream.push(Self::Atom(it.next().unwrap())),
        }
}
