use linefeed::{Interface, ReadResult};

use rusht::parse::Ast;

const PROMPT: &str = "rusht> ";
const HISTORY_FILE_NAME: &str = ".rusht_history";


fn main() -> std::io::Result<()> {
    let history_file = dirs::home_dir()
        .map(|d| d.join(HISTORY_FILE_NAME))
        .expect("failed to construct history file path");

    let reader = Interface::new("rusht")?;

    reader.set_prompt(PROMPT)?;
    reader.load_history(&history_file)?;

    while let ReadResult::Input(input) = reader.read_line()? {
        reader.add_history_unique(input.clone());

        let tokens = rusht::tokenize::tokenize(input.as_str());
        let ast = rusht::parse::parse(tokens);
        match ast {
            Ast::List(x) => println!("{:?}", rusht::interpret::interpret(x.get(0).unwrap().clone())),
            _ => panic!("asdf")
        }
    }

    reader.save_history(&history_file)?;

    Ok(())
}
