use clap::Parser;
use sequence::sequence;

#[derive(Parser)]
struct Input {
    #[arg(short, long, default_value_t = 500)]
    length: usize,
    #[arg(short, long, default_value_t = 1000)]
    ceiling: usize,
    #[arg(short, long, default_value_t = false)]
    duplicate: bool,
    initial: Vec<usize>,
}

fn main() {
    let input = Input::parse();

    let iterator: Box<dyn Iterator<Item = usize>> =
        sequence(input.initial, input.ceiling, input.duplicate);
    let seq: Vec<usize> = iterator.take(input.length).collect();

    println!("{} {:?}", seq.len(), seq);
}
