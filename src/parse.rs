use std::slice::Iter;

use crate::tokenize::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Ast {
    Atom(Token),
    List(Vec<Ast>),
}

pub fn parse(tokens: Vec<Token>) -> Ast {
    parse_it(&mut tokens.iter())
}

fn parse_it(it: &mut Iter<Token>) -> Ast {
    let mut items = vec![];
    while let Some(token) = it.next() {
        let item = match token {
            Token::Paren('(') => {
                parse_it(it)
            },
            Token::Paren(')') => break,
            _ => Ast::Atom(token.clone())
        };
        items.push(item);
    }
    Ast::List(items)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Token::*;

    macro_rules! test_parse {
        ($($name:ident: $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let out = parse($input);
                    assert_eq!(out, $expected);
                }
            )*
        };
    }

    test_parse!(
        test_empty: vec![Paren('('), Paren(')')] => Ast::List(vec![Ast::List(vec![])]),
        test_single: vec![Paren('('), Num(4.0), Paren(')')] => Ast::List(vec![Ast::List(vec![Ast::Atom(Num(4.0))])]),
        test_nested: vec![Paren('('), Num(4.0), Paren('('), Num(5.0), Str("foo".to_string()), Paren(')'), Paren(')')] => Ast::List(vec![Ast::List(vec![Ast::Atom(Num(4.0)), Ast::List(vec![Ast::Atom(Num(5.0)), Ast::Atom(Str("foo".to_string()))])])])
    );
}