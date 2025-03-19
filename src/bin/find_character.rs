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

    let iterator: Box<dyn Iterator<Item = usize>> =
        sequence(input.initial, input.ceiling, input.duplicate);
    let seq: Vec<usize> = iterator.take(input.length).collect();
    if input.verbose {
        println!("{} {:?}", seq.len(), seq);
    }

    match determine_character(&seq) {
        Option::Some(character) => println!("{}", character),
        Option::None => println!("?"),
    }
}
