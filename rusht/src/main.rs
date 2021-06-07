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
}
