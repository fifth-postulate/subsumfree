use clap::Parser;
use sequence::combinatorics::combination::Sequence as CombinationSequence;
use sequence::word::expression::Sequence as ExpressionSequence;

#[derive(Parser)]
struct Input {
    #[arg(short, long, default_value_t = 500)]
    length: usize,
    #[arg(short, long, default_value_t = 1000)]
    ceiling: usize,
    #[arg(short, long, default_value_t = false)]
    duplicate: bool,
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

    println!("{} {:?}", seq.len(), seq);
}
