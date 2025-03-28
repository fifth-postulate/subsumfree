//! A [Combination](https://en.wikipedia.org/wiki/Combination) is
//!
//! > a selection of items from a set that has distinct members, such that the
//! > order of selection does not matter.
//!
//! The `combination` module allows to iterate over all _characteristic words_
//! of all combinations of certain number of elements.
//!
//! ```
//! # use sequence::combinatorics::Combinations;
//! let actual = Combinations::new(5,2).count();
//! let expected = 10;
//! assert_eq!(actual, expected);
//! ```

mod sequence;

pub use sequence::Sequence;

/// An iterator for characteristic words of combinations.
#[derive(Debug, PartialEq, Eq)]
pub struct Combinations {
    t: usize,
    current: Option<Vec<usize>>,
}

impl Combinations {
    /// Creates an iterator that iterates over all characteristic words of
    /// combinations of n elements where t are chosen.
    pub fn new(n: usize, t: usize) -> Self {
        let mut current: Vec<usize> = vec![0; n];
        if n > 0 {
            for index in 0..t {
                current[index] = 1;
            }
        }
        Self {
            t,
            current: Option::Some(current),
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Option::Some(current) => {
                let mut combination: Vec<usize> = current.to_vec();
                let result = Option::Some(combination.to_vec());
                let mut index = combination.len() - 1;
                while index > 0 && combination[index] == 0 {
                    index -= 1;
                }
                if index == combination.len() - 1 {
                    let mut count = 0;
                    while index > 0 && combination[index] == 1 {
                        combination[index] = 0;
                        index -= 1;
                        count += 1;
                    }
                    if index == 0 && combination[index] == 1 {
                        count += 1;
                    }
                    if count < self.t {
                        while index > 0 && combination[index] == 0 {
                            index -= 1;
                        }
                        combination[index] = 0;
                        index += 1;
                        while count > 0 {
                            combination[index] = 1;
                            index += 1;
                            count -= 1;
                        }
                        combination[index] = 1;
                        self.current = Option::Some(combination)
                    } else {
                        self.current = Option::None
                    }
                } else {
                    if combination[index] == 1 {
                        combination[index] = 0;
                        combination[index + 1] = 1;
                        self.current = Option::Some(combination)
                    } else {
                        self.current = Option::None
                    }
                }
                result
            }
            Option::None => Option::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Combinations;

    #[test]
    fn combinations_4_2_generates_all_possibilities() {
        let actual: Vec<Vec<usize>> = Combinations::new(4, 2).collect();
        let expected = vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 1, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 1],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn combinations_5_3_generates_possibilities() {
        let actual: Vec<Vec<usize>> = Combinations::new(5, 3).collect();
        let expected = vec![
            vec![1, 1, 1, 0, 0],
            vec![1, 1, 0, 1, 0],
            vec![1, 1, 0, 0, 1],
            vec![1, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 1, 1],
            vec![0, 1, 1, 1, 0],
            vec![0, 1, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![0, 0, 1, 1, 1],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn combinatations_3_3_generates_all_possibilities() {
        let actual: Vec<Vec<usize>> = Combinations::new(3, 3).collect();
        let expected = vec![vec![1, 1, 1]];

        assert_eq!(actual, expected);
    }
}
