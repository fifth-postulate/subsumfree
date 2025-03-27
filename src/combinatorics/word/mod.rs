//! A word is a sequence of weights of a certain length and weight.
mod sequence;

pub use sequence::Sequence;

/// An iterator for all words of certain length and weight.
#[derive(Debug, PartialEq, Eq)]
pub struct Words {
    weight: usize,
    current: Option<Vec<usize>>,
}

impl Words {
    /// Creates an iterator for all words of certain length and weight.
    pub fn new(length: usize, weight: usize) -> Self {
        let mut current: Vec<usize> = vec![0; length];
        if length > 0 {
            current[0] = weight;
        }
        Self {
            weight,
            current: Option::Some(current),
        }
    }
}

impl Iterator for Words {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Option::Some(current) => {
                let mut current_word: Vec<usize> = current.to_vec();
                let result = Option::Some(current_word.to_vec());
                if current_word.len() > 0 {
                    let n = current_word.len() - 1;
                    let mut index = n;
                    if current_word[index] < self.weight {
                        let residue = current_word[index];
                        current_word[index] = 0;
                        while index > 0 && current_word[index] == 0 {
                            index -= 1;
                        }

                        if current_word[index] > 1 {
                            current_word[index] -= 1;
                            current_word[index + 1] = residue + 1;
                        } else {
                            current_word[index] = 0;
                            current_word[index + 1] = residue + 1;
                        }
                        self.current = Option::Some(current_word);
                    } else {
                        self.current = Option::None;
                    }
                } else {
                    self.current = Option::None;
                }
                result
            }
            Option::None => Option::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Words;

    #[test]
    fn words_4_3_generates_all_possibilities() {
        let actual: Vec<Vec<usize>> = Words::new(4, 3).collect();
        let expected = vec![
            vec![3, 0, 0, 0],
            vec![2, 1, 0, 0],
            vec![2, 0, 1, 0],
            vec![2, 0, 0, 1],
            vec![1, 2, 0, 0],
            vec![1, 1, 1, 0],
            vec![1, 1, 0, 1],
            vec![1, 0, 2, 0],
            vec![1, 0, 1, 1],
            vec![1, 0, 0, 2],
            vec![0, 3, 0, 0],
            vec![0, 2, 1, 0],
            vec![0, 2, 0, 1],
            vec![0, 1, 2, 0],
            vec![0, 1, 1, 1],
            vec![0, 1, 0, 2],
            vec![0, 0, 3, 0],
            vec![0, 0, 2, 1],
            vec![0, 0, 1, 2],
            vec![0, 0, 0, 3],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn words_3_5_generates_all_possibilities() {
        let actual: Vec<Vec<usize>> = Words::new(3, 5).collect();
        let expected = vec![
            vec![5, 0, 0],
            vec![4, 1, 0],
            vec![4, 0, 1],
            vec![3, 2, 0],
            vec![3, 1, 1],
            vec![3, 0, 2],
            vec![2, 3, 0],
            vec![2, 2, 1],
            vec![2, 1, 2],
            vec![2, 0, 3],
            vec![1, 4, 0],
            vec![1, 3, 1],
            vec![1, 2, 2],
            vec![1, 1, 3],
            vec![1, 0, 4],
            vec![0, 5, 0],
            vec![0, 4, 1],
            vec![0, 3, 2],
            vec![0, 2, 3],
            vec![0, 1, 4],
            vec![0, 0, 5],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn words_of_length_zero_produce_the_empty_word() {
        let actual: Vec<Vec<usize>> = Words::new(0, 37).collect();
        let expected = vec![vec![]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn words_of_weight_zero_only_produce_the_zero_word() {
        let actual: Vec<Vec<usize>> = Words::new(4, 0).collect();
        let expected = vec![vec![0, 0, 0, 0]];
        assert_eq!(actual, expected);
    }
}
