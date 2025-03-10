//! The `period` module allow the detection of cycles.

use std::fmt::Display;

/// Provides information about the period
#[derive(Debug, PartialEq, Eq)]
pub struct Info {
    /// The length of the pre-period
    pub pre_period: usize,
    /// The length of the period.
    pub period: usize,
}

impl Info {
    /// Create `Info` with the pre-period and period.
    pub fn new(pre_period: usize, period: usize) -> Self {
        Self { pre_period, period }
    }

    /// Check if a sequence adheres to the `Info`.
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

/// Try to find the `Info` of this sequence.
pub fn detect_cycle(elements: &[usize]) -> Option<Info> {
    for pre_period in 0..(elements.len() / 2) {
        for period in 1..(elements.len() / 2) {
            if elements
                .iter()
                .skip(pre_period + period)
                .zip(elements.iter().skip(pre_period))
                .all(|(l, r)| l == r)
            {
                return Option::Some(Info::new(pre_period, period));
            }
        }
    }
    Option::None
}

#[cfg(test)]
mod tests {
    use super::{Info, detect_cycle};

    #[test]
    fn monotonic_increasing_sequence_does_not_have_a_cycle() {
        let sequence: Vec<usize> = (0..100).collect();

        let info = detect_cycle(&sequence);

        assert!(info.is_none());
    }

    #[test]
    fn a_cycle_is_detected_with_correct_info() {
        let sequence: Vec<usize> = (0..3).cycle().take(100).collect();

        match detect_cycle(&sequence) {
            Option::Some(info) if info.check(&sequence) => {
                assert_eq!(info, Info::new(0, 3));
            }
            _ => {
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
