<<<<<<< HEAD
//mod lib;
//use crate::interpret::interpret;

//mod interpret;
use interpret::interpret;

mod parse;
//use parse::Ast;

mod tokenize;
//use tokenize::Token;

fn main() {

    // zum Testen
    let input = "(+ 3 4)";
    let tokenstream = tokenize::tokenize(input);
    let tree = parse::parse(tokenstream);
    println!("our tree: {:?}\n", tree);

    let result = interpret::interpret(tree);
    println!("our solution: {:?}\n", tree);


    // new string for input console
    // let mut line_terminal = String::new();
    // to read all lines - necessary?
    // let print_line = std::io::stdin().read_line(&mut line_terminal).unwrap();
    // println!("Input from Console: {}", print_line);
=======
use std::io::Write;

use rusht::interpret::interpret;
use rusht::parse::{Ast, parse};
use rusht::tokenize::tokenize;

fn main() {
    print!("> ");
    std::io::stdout().flush().expect("failed to flush");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let tokens = tokenize(buf.as_str());
    let ast = parse(tokens);
    println!("{:?}", interpret(match ast {
        Ast::List(l) => l.get(0).unwrap().clone(),
        _ => panic!("should be nested")
    }));
>>>>>>> master
}
