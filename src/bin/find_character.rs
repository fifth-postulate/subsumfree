use clap::Parser;
use sequence::Sequence;
use sequence::character::determine_character;

#[derive(Parser)]
struct Input {
    #[arg(short, long, default_value_t = 500)]
    length: usize,
    #[arg(short, long, default_value_t = 1000)]
    ceiling: usize,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    a: usize,
    b: usize,
    c: usize,
}

fn main() {
    let input = Input::parse();

    let seq: Vec<usize> = Sequence::with_maximum(vec![input.a, input.b, input.c], input.ceiling)
        .take(input.length)
        .collect();
    if input.verbose {
        println!("{} {:?}", seq.len(), seq);
    }

    match determine_character(&seq) {
        Option::Some(character) => println!("{}", character),
        Option::None => println!("?"),
    }
}
