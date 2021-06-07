use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{App, Arg};
use linefeed::{DefaultTerminal, Interface, ReadResult};

use rusht::parse::Ast;

const PROGRAM_NAME: &str = "rusht";
const REPL_PROMPT: &str = "rusht> ";
const REPL_HISTORY_FILE_NAME: &str = ".rusht_history";
const REPL_HISTORY_SIZE: usize = 100;


fn main() -> Result<()> {
    let matches = App::new(PROGRAM_NAME)
        .version("0.1.0")
        .author("Isabella Schön, Marc Trölitzsch")
        .arg(Arg::new("FILE").about("program read from script file"))
        .get_matches();

    match matches.value_of("FILE") {
        None => start_repl(),
        Some(file) => interpret_file(file)
    }
}

fn interpret_file(file_name: &str) -> Result<()> {
    let src = std::fs::read_to_string(file_name)
        .context("failed to read program from file")?;

    let result = interpret(src).context("failed to interpret file")?;
    println!("{:?}", result);

    Ok(())
}

fn start_repl() -> Result<()> {
    let reader = init_reader()
        .context("failed to initialize reader")?;

    while let ReadResult::Input(input) = reader.read_line().context("failed to read line")? {
        reader.add_history(input.clone());

        interpret(input)
            .map(|result| println!("{:?}", result))
            .context("failed to interpret line")?;
    }

    reader.save_history(history_file_path()?)
        .context("failed to write history")?;

    Ok(())
}

fn init_reader() -> Result<Interface<DefaultTerminal>> {
    let reader = Interface::new(PROGRAM_NAME)
        .context("failed to get terminal interface")?;

    reader.set_prompt(REPL_PROMPT).context("failed to set prompt")?;
    reader.set_history_size(REPL_HISTORY_SIZE);
    match reader.load_history(history_file_path()?) {
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            // load_history will return an error, if no history file exists. We are explicitly
            // ignoring this error, as we are fine with not loading any history in that case.
            // The file will be created after the repl has terminated for the first time and will be
            // available on the next run.
            Ok(())
        }
        result => result
    }?;

    Ok(reader)
}

fn history_file_path() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|d| d.join(REPL_HISTORY_FILE_NAME))
        .context("failed to construct history file path")
}

fn interpret(src: String) -> Result<rusht::tokenize::Token> {
    let tokens = rusht::tokenize::tokenize(src.as_str());
    match rusht::parse::parse(tokens) {
        Ast::Atom(_) => bail!("expected to get an Ast::List, got Ast::Atom"),
        Ast::List(list) => {
            let ast = list.get(0).unwrap().clone();
            Ok(rusht::interpret::interpret(ast))
        }
    }
}
