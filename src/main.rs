use clap::Parser;
use sequence::Sequence;

#[derive(Parser)]
struct Input {
    #[arg(short, long, default_value_t = 500)]
    length: usize,
    #[arg(short, long, default_value_t = 1000)]
    ceiling: usize,
    a: usize,
    b: usize,
    c: usize,
}

fn main() {
    let input = Input::parse();

    let seq: Vec<usize> = Sequence::with_maximum(input.a, input.b, input.c, input.ceiling)
        .take(input.length)
        .collect();

    println!("{} {:?}", seq.len(), seq);
}
