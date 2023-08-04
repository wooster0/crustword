use super::Error;
use std::{env, fs, iter::Peekable};

/// The raw form of the word list.
pub type ArgWords = Peekable<env::Args>;

/// The command to be executed.
pub enum Command {
    Generate { watch: bool, arg_words: ArgWords },
    Solve { watch: bool, crossword_str: String },
}

pub fn parse() -> Result<Command, Error> {
    let mut args = env::args().peekable();

    // We will assume the first argument is the executable path
    if args.next().is_none() {
        return Err("expected executable path");
    };

    let watch = if args.peek().map(|string| string.as_ref()) == Some("watch") {
        args.next();
        true
    } else {
        false
    };

    if let Some(arg) = args.next() {
        match arg.as_ref() {
            "gen" => Ok(Command::Generate { watch, arg_words: args }),
            "solve" => {
                if let Some(file_name) = args.next() {
                    if let Ok(crossword_str) = fs::read_to_string(file_name) {
                        Ok(Command::Solve { watch, crossword_str })
                    } else {
                        Err("error reading file")
                    }
                } else {
                    Err("no file name")
                }
            }
            _ => Err("invalid command"),
        }
    } else {
        Err("* Use `gen` followed by a list of words as the arguments to generate a crossword containing those words.\n  Use `watch gen` followed by words to watch a crossword being made out of those words.\n* Use `solve` followed by the path to a crossword as the arguments to solve a crossword.\n  Use `watch solve` followed by a crossword to watch it being solved.\nExample: `crustword watch solve crosswords/generated`")
    }
}
