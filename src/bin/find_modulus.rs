use clap::Parser;
use sequence::Sequence;
use sequence::period::detect_cycle;

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

    for modules in input.min..input.max {
        let mod_seq: Vec<usize> = seq.iter().map(|n| n % modules).collect();
        match detect_cycle(&mod_seq) {
            Some(info) if info.check(&mod_seq) => {
                print!("{}: {}", modules, info);
                if input.verbose {
                    print!("{:?}", mod_seq);
                }
                println!();
            }
            _ => {
                // do nothing
            }
        }
    }
}
