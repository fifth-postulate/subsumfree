mod sequence;

pub use sequence::Sequence;

#[derive(Debug, PartialEq, Eq)]
pub struct Words {
    weight: usize,
    current: Option<Vec<usize>>,
}

impl Words {
    pub fn new(length: usize, weight: usize) -> Self {
        let mut current: Vec<usize> = vec![0; length];
        if length > 0 {
            current[length - 1] = 1;
            current[0] = weight - 1
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
                let n = current_word.len() - 1;
                let mut index = n;
                if current_word[index] < self.weight {
                    index -= 1;
                    while index > 0 && current_word[index] == 0 {
                        index -= 1;
                    }
                    if current_word[index] > 1 || index == n - 1 {
                        current_word[index] -= 1;
                        current_word[index + 1] += 1;
                    } else {
                        current_word[index] = 0;
                        current_word[index + 1] = 1;
                        current_word.swap(index + 1, n);
                    }
                    self.current = Option::Some(current_word);
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
            vec![2, 0, 0, 1],
            vec![1, 1, 0, 1],
            vec![1, 0, 1, 1],
            vec![1, 0, 0, 2],
            vec![0, 2, 0, 1],
            vec![0, 1, 1, 1],
            vec![0, 1, 0, 2],
            vec![0, 0, 2, 1],
            vec![0, 0, 1, 2],
            vec![0, 0, 0, 3],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn words_3_4_generates_all_possibilities() {
        let actual: Vec<Vec<usize>> = Words::new(3, 5).collect();
        let expected = vec![
            vec![4, 0, 1],
            vec![3, 1, 1],
            vec![3, 0, 2],
            vec![2, 1, 2],
            vec![2, 0, 3],
            vec![1, 1, 3],
            vec![1, 0, 4],
            vec![0, 4, 1],
            vec![0, 3, 2],
            vec![0, 2, 3],
            vec![0, 1, 4],
            vec![0, 0, 5],
        ];
        assert_eq!(actual, expected);
    }
}
