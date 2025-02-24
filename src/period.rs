#[derive(Debug, PartialEq, Eq)]
pub struct Info {
    pre_period: usize,
    period: usize,
}

impl Info {
    pub fn new(pre_period: usize, period: usize) -> Self {
        Self { pre_period, period }
    }
}

/// The `detect_cycle` tries to find a cycle if it exists.
///
/// It uses [floyds detection algorithm](https://en.wikipedia.org/wiki/Cycle_detection)
pub fn detect_cycle(elements: &[usize]) -> Option<Info> {
    let (mut tortoise, mut hare) = (1, 2);

    while hare < elements.len() && elements[tortoise] != elements[hare] {
        tortoise += 1;
        hare += 2;
    }

    if hare < elements.len() {
        // There is a cycle
        tortoise = 0;
        while hare < elements.len() && elements[tortoise] != elements[hare] {
            tortoise += 1;
            hare += 1;
        }
        if hare < elements.len() {
            let pre_period = tortoise;
            hare = tortoise + 1;
            let mut period = 1;
            while elements[tortoise] != elements[hare] {
                hare += 1;
                period += 1;
            }
            Option::Some(Info::new(pre_period, period))
        } else {
            // Whoops, there is a cycle but ran out of elements for floyd algorithm.
            Option::None
        }
    } else {
        Option::None
    }
}

mod tests {
    use super::*;

    #[test]
    fn monotonic_increasing_sequence_does_not_have_a_cycle() {
        let sequence: Vec<usize> = (0..100).collect();

        let info = detect_cycle(&sequence);

        assert!(info.is_none())
    }

    #[test]
    fn a_cycle_is_detected_with_correct_info() {
        let sequence: Vec<usize> = (0..3).cycle().take(100).collect();

        match detect_cycle(&sequence) {
            Option::Some(info) => {
                println!("{:?}", info);
                assert_eq!(info, Info::new(0, 3));
            }
            Option::None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn a_cycle_with_a_pre_period_is_detected_with_correct_info() {
        let pre: Vec<usize> = (0..5).collect();
        let period: Vec<usize> = (0..3).cycle().take(100).collect();
        let sequence: Vec<usize> = pre.iter().chain(period.iter()).cloned().collect();

        match detect_cycle(&sequence) {
            Option::Some(info) => {
                println!("{:?}", info);
                assert_eq!(info, Info::new(5, 3));
            }
            Option::None => {
                assert!(false);
            }
        }
    }
}
