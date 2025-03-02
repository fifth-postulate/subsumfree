use clap::Parser;
use sequence::period::detect_cycle;
use sequence::word::Sequence;

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
    a: usize,
    b: usize,
    c: usize,
}

fn main() {
    let input = Input::parse();

    for a in 1..=input.a {
        for b in (a + 1)..=input.b {
            for c in (b + 1)..=input.c {
                let seq: Vec<usize> = Sequence::with_maximum(vec![a, b, c], input.ceiling)
                    .take(input.length)
                    .collect();
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
