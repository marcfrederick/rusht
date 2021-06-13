use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use clap::{App, Arg};
use linefeed::{Command, DefaultTerminal, Function, Interface, Prompter, ReadResult, Terminal};

use rusht::{Interpreter, Token};

const PROGRAM_NAME: &str = "rusht";
const REPL_PROMPT: &str = "rusht> ";
const REPL_HISTORY_FILE_NAME: &str = ".rusht_history";
const REPL_HISTORY_SIZE: usize = 100;


struct RushtAccept;

impl<Term: Terminal> Function<Term> for RushtAccept {
    fn execute(&self, prompter: &mut Prompter<Term>, count: i32, _ch: char) -> std::io::Result<()> {
        // TODO: Match out for specific errors (unclosed paren, ...) and do either
        //  `prompter.accept_input()` or ` prompter.insert(count as usize, '\n')`
        let buf = prompter.buffer().to_string();

        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_info| {}));
        let result = std::panic::catch_unwind(|| {
            interpret(buf).expect("")
        });
        std::panic::set_hook(hook);

        if result.is_ok() {
            prompter.accept_input()
        } else {
            prompter.insert(count as usize, '\n')
        }
    }
}


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

/// Interprets the code at the given file path.
fn interpret_file(file_path: &str) -> Result<()> {
    let result = std::fs::read_to_string(file_path)
        .context("failed to read program from file")
        .and_then(interpret)
        .context("failed to interpret file")?;

    println!("{}", result);
    Ok(())
}

/// Starts a new REPL.
fn start_repl() -> Result<()> {
    let reader = init_reader()
        .context("failed to initialize reader")?;

    while let ReadResult::Input(input) = reader.read_line().context("failed to read line")? {
        reader.add_history(input.clone());

        interpret(input)
            .map(|result| println!("{}", result))
            .context("failed to interpret line")?;
    }

    if let Some(p) = history_file_path() {
        reader.save_history(p)
            .context("failed to write history")?;
    }

    Ok(())
}

/// Returns an initialized terminal interface.
///
/// The returned value is either an `Ok`, containing an initialized interface, or an `Err`.
fn init_reader() -> Result<Interface<DefaultTerminal>> {
    let reader = Interface::new(PROGRAM_NAME)
        .context("failed to get terminal interface")?;

    reader.set_prompt(REPL_PROMPT).context("failed to set prompt")?;
    reader.set_history_size(REPL_HISTORY_SIZE);
    if let Some(p) = history_file_path() {
        reader.load_history(p).context("failed to load history")?
    }

    reader.define_function("rusht-accept", Arc::from(RushtAccept));
    reader.bind_sequence("\n", Command::from_str("rusht-accept"));
    reader.bind_sequence("\r", Command::from_str("rusht-accept"));
    {
        let mut reader = reader.lock_reader();
        reader.blink_matching_paren();
        reader.set_string_chars("\"");
    }

    Ok(reader)
}

/// Returns the path to the REPL history.
///
/// The returned value depends on the operating system and is either a `Some`, containing the path
/// of an existing history file, or a `None`.
fn history_file_path() -> Option<PathBuf> {
    dirs::home_dir()
        .map(|d| d.join(REPL_HISTORY_FILE_NAME))
        .filter(|p| p.exists())
}

/// Interprets the given `String` and returns the resulting `Token`.
fn interpret(src: String) -> Result<Token> {
    Interpreter::new()
        .interpret(src.as_str())
        .context("failed to interpret input")
}
