use std::collections::HashMap;

use crate::Env;
use crate::tokenize::Token;

macro_rules! prelude {
    ($($key:expr => $val:expr),*) => {
        {
            let mut hash_map: HashMap<String, fn(Vec<Token>) -> Token> = HashMap::new();
            $(
                hash_map.insert($key.to_string(), $val);
            )*
            hash_map
        }
    };
}

pub fn get_prelude() -> Env {
    prelude!(
        "+" => add,
        "add" => add,
        "-" => sub,
        "sub" => sub,
        "*" => mul,
        "mul" => mul,
        "/" => div,
        "div" => div
    )
}

fn add(args: Vec<Token>) -> Token {
    let mut sum = 0.0;
    for x in args {
        match x {
            Token::Num(n) => sum += n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

fn sub(args: Vec<Token>) -> Token {
    let mut sum: f64 = match args[0] {
        Token::Num(n) => n + n,
        _ => panic!("Not a number")
    };
    for x in args {
        match x {
            Token::Num(n) => sum -= n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

fn mul(args: Vec<Token>) -> Token {
    let mut sum = 1.0;
    for x in args {
        match x {
            Token::Num(n) => sum *= n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

fn div(args: Vec<Token>) -> Token {
    let mut sum: f64 = match args[0] {
        Token::Num(n) => n * n,
        _ => panic!("Not a number")
    };
    for x in args {
        match x {
            Token::Num(n) => sum /= n,
            _ => panic!("Not a number")
        }
    }
    Token::Num(sum)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Token::*;

    macro_rules! test_fn {
        ($($func:ident - $name:ident: $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($func($input), $expected);
                }
            )*
        };
    }

    test_fn! {
        add - add_two: vec![Num(1.0), Num(3.0)] => Num(4.0),
        add - add_three: vec![Num(1.0), Num(3.0), Num(10.0)] => Num(14.0),
        sub - sub_two: vec![Num(1.0), Num(3.0)] => Num(-2.0),
        sub - sub_four: vec![Num(1.0), Num(3.0), Num(2.0), Num(5.0)] => Num(-9.0),
        mul - mul_two: vec![Num(2.0), Num(3.0)] => Num(6.0),
        mul - mul_three: vec![Num(1.0), Num(3.0), Num(10.0)] => Num(30.0),
        div - div_two: vec![Num(3.0), Num(2.0)] => Num(1.5),
        div - div_three: vec![Num(3.0), Num(2.0), Num(0.5)] => Num(3.0)
    }
}
