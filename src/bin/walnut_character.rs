use std::io::{self};

use clap::Parser;
use sequence::sequence;
use sequence::tools::character::determine_character;

#[derive(Parser)]
struct Input {
    #[arg(short, long, default_value_t = 500)]
    length: usize,
    #[arg(short, long, default_value_t = 1000)]
    ceiling: usize,
    #[arg(short, long, default_value_t = false)]
    duplicate: bool,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    initial: Vec<usize>,
}

fn main() {
    let input = Input::parse();

    let seq: Vec<usize> = sequence(input.initial, input.ceiling, input.duplicate)
        .take(input.length)
        .collect();
    if input.verbose {
        println!("{} {:?}", seq.len(), seq);
    }

    match determine_character(&seq) {
        Option::Some(character) => {
            let _ = character.write_walnut(&"seq", &mut io::stdout());
        }
        Option::None => println!("?"),
    }
}
