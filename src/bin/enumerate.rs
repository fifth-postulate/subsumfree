use clap::Parser;
use sequence::Sequence;
use sequence::period::detect_cycle;

#[derive(Parser)]
struct Input {
    #[arg(long, default_value_t = 1_000_000)]
    max_length: usize,
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
            'candidate: for c in (b + 1)..=input.c {
                let mut length = 10;
                while length < input.max_length {
                    let seq: Vec<usize> = Sequence::new(a, b, c).take(length).collect();
                    for modules in input.min..input.max {
                        let mod_seq: Vec<usize> = seq.iter().map(|n| n % modules).collect();
                        match detect_cycle(&mod_seq) {
                            Some(info) if info.check(&mod_seq) => {
                                println!("{} {} {}: {} {}", a, b, c, modules, info);
                                continue 'candidate;
                            }
                            _ => {
                                // do nothing
                            }
                        }
                    }
                    length *= 10;
                }
                println!("{} {} {}: ?", a, b, c);
            }
        }
    }
}
