use clap::Parser;
use sequence::Sequence;
use sequence::period::detect_cycle;

#[derive(Parser)]
struct Input {
    #[arg(short, long, default_value_t = 500)]
    length: usize,
    #[arg(short, long, default_value_t = 1000)]
    ceiling: usize,
    #[arg(long, default_value_t = 1)]
    start: usize,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    a: usize,
    b: usize,
    c: usize,
}

fn main() {
    let input = Input::parse();

    let seq: Vec<usize> = Sequence::with_maximum(input.a, input.b, input.c, input.ceiling)
        .take(input.length)
        .collect();
    if input.verbose {
        println!("{} {:?}", seq.len(), seq);
    }

    let differences: Vec<usize> = seq.windows(2).map(|t| t[1] - t[0]).collect();
    if input.verbose {
        println!("{} {:?}", differences.len(), differences);
    }

    match detect_cycle(&differences) {
        Some(info) if info.check(&differences) => {
            println!("{}", info);
        }
        _ => println!("?"),
    }
}
