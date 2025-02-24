use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Info {
    pre_period: usize,
    period: usize,
}

impl Info {
    pub fn new(pre_period: usize, period: usize) -> Self {
        Self { pre_period, period }
    }

    pub fn check(&self, elements: &[usize]) -> bool {
        for index in (self.pre_period + self.period)..elements.len() {
            if elements[index] != elements[index - self.period] {
                return false;
            }
        }
        true
    }
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.pre_period, self.period)
    }
}

pub fn detect_cycle(elements: &[usize]) -> Option<Info> {
    detect_cycle_from(1, elements)
}

/// The `detect_cycle` tries to find a cycle if it exists.
///
/// It uses [floyds detection algorithm](https://en.wikipedia.org/wiki/Cycle_detection)
fn detect_cycle_from(start: usize, elements: &[usize]) -> Option<Info> {
    let (mut tortoise, mut hare) = (start, 2 * start);

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

    #[test]
    fn info_can_check_if_an_sequence_adheres() {
        let info = Info::new(3, 7);
        let passes = vec![0, 1, 2, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7];
        let fails = vec![0, 1, 2, 1, 2, 3, 4, 5, 6, 7, 1, 2, 37, 4, 5, 6, 7];

        assert!(info.check(&passes));
        assert!(!info.check(&fails));
    }
}
