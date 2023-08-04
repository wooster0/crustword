//! Crusty Crosswords

mod args;
mod crossword;
mod util;

use args::Command;
use crossword::Crossword;
use std::{io, process};

type Error = &'static str;

fn main() {
    let command = args::parse();

    match command {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
        Ok(Command::Generate { watch, arg_words }) => {
            let stdout = io::stdout();
            let mut lock = stdout.lock();
            match crossword::gen(watch, arg_words, &mut lock) {
                Err(err) => {
                    eprintln!("crossword generation error: {}", err);
                    process::exit(1);
                }
                Ok(()) => {}
            }
        }
        Ok(Command::Solve {
            watch,
            crossword_str,
        }) => {
            let crossword = Crossword::try_from(crossword_str.as_ref());
            match crossword {
                Err(err) => {
                    eprintln!("crossword parsing error: {}", err);
                    process::exit(1);
                }
                Ok(mut crossword) => {
                    println!("Before:\n{}", crossword);

                    crossword.solve(watch);

                    println!("After:\n{}", crossword);
                }
            }
        }
    }
}
