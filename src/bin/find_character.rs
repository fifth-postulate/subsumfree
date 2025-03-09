use clap::Parser;
use sequence::combinatorics::combination::Sequence as CombinationSequence;
use sequence::combinatorics::word::Sequence as ExpressionSequence;
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
    a: usize,
    b: usize,
    c: usize,
}

fn main() {
    let input = Input::parse();

    let iterator: Box<dyn Iterator<Item = usize>> = if input.duplicate {
        Box::new(ExpressionSequence::with_maximum(
            vec![input.a, input.b, input.c],
            input.ceiling,
        ))
    } else {
        Box::new(CombinationSequence::with_maximum(
            vec![input.a, input.b, input.c],
            input.ceiling,
        ))
    };
    let seq: Vec<usize> = iterator.take(input.length).collect();
    if input.verbose {
        println!("{} {:?}", seq.len(), seq);
    }

    match determine_character(&seq) {
        Option::Some(character) => println!("{}", character),
        Option::None => println!("?"),
    }
}
