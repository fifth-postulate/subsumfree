use sequence::period::detect_cycle;
use sequence::{Sequence, express};

fn main() {
    let seq: Vec<usize> = Sequence::new(1, 3, 5).take(30).collect();
    println!("{:?}", seq);

    for m in 2..10 {
        let mod_seq: Vec<usize> = seq.iter().map(|n| n % m).collect();
        match detect_cycle(&mod_seq) {
            Some(info) => {
                println!("{:?}: ({:?}) {:?}", m, info, mod_seq);
            }
            None => {
                println!("{:?}: {:?}", m, mod_seq);
            }
        }
    }

    let result = express(20, &vec![1, 3, 5, 6, 7, 8]);
    println!("{:?}", result)
}
