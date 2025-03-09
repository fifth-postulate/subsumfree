use clap::Parser;
use sequence::combinatorics::combination::Sequence as CombinationSequence;
use sequence::combinatorics::word::Sequence as ExpressionSequence;
use sequence::tools::period::detect_cycle;

#[derive(Parser)]
struct Input {
    #[arg(short, long, default_value_t = 500)]
    length: usize,
    #[arg(short, long, default_value_t = 1000)]
    ceiling: usize,
    #[arg(long, default_value_t = 2)]
    min: usize,
    #[arg(long, default_value_t = 50)]
    max: usize,
    #[arg(short, long, default_value_t = false)]
    duplicate: bool,
    a: usize,
    b: usize,
    c: usize,
}

fn main() {
    let input = Input::parse();

    for a in 1..=input.a {
        for b in (a + 1)..=input.b {
            for c in (b + 1)..=input.c {
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
                let mut found = false;
                for modules in input.min..input.max {
                    let mod_seq: Vec<usize> = seq.iter().map(|n| n % modules).collect();
                    match detect_cycle(&mod_seq) {
                        Some(info) if info.check(&mod_seq) => {
                            println!("{} {} {}: {} {}", a, b, c, modules, info);
                            found = true;
                        }
                        _ => {
                            // do nothing
                        }
                    }
                }
                if !found {
                    println!("{} {} {}: ?", a, b, c);
                }
            }
        }
    }
}
