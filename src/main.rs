use sequence::period::detect_cycle;
use sequence::{Sequence, express};

fn main() {
    let seq: Vec<usize> = Sequence::new(1, 3, 5).take(100).collect();
    println!("{:?}", seq);

    for m in 2..20 {
        let mod_seq: Vec<usize> = seq.iter().map(|n| n % m).collect();
        match detect_cycle(&mod_seq) {
            Some(info) if info.check(&mod_seq) => {
                println!("{:?}: ({:?}) {:?}", m, info, mod_seq);
            }
            _ => {
                println!("{:?}: not periodic", m);
            }
        }
    }
}
