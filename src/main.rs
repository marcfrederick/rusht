mod parse;

fn main() {

    // zum Testen
    let input = "(+ 2 3 4)";
    Token token = Tokenizer::new(input);
    println!("{:?}", token);



    // new string for input console
    let mut line_terminal = String::new();
    // to read all lines - necessary?
    let print_line = std::io::stdin().read_line(&mut line_terminal).unwrap();
    //println!("Input from Console: {}", print_line);
    //Token token = Tokenizer::new(print_line);
    println("{:?} :-) ", result);
}
