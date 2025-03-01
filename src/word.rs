pub struct Combinations {
    t: usize,
    current: Option<Vec<usize>>,
}

impl Combinations {
    pub fn new(n: usize, t: usize) -> Self {
        let mut current: Vec<usize> = vec![0; n];
        if n > 0 {
            current[n - 1] = 1;
            for index in 0..(t - 1) {
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
                let mut index = combination.len() - 2;
                let mut count = 1;
                while index > 0 && combination[index] == 1 {
                    combination[index] = 0;
                    index -= 1;
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
                    self.current = Option::Some(combination)
                } else {
                    self.current = Option::None
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
    fn combinations_4_2_generates_possibilities() {
        let actual: Vec<Vec<usize>> = Combinations::new(4, 2).collect();
        let expected = vec![vec![1, 0, 0, 1], vec![0, 1, 0, 1], vec![0, 0, 1, 1]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn combinations_5_3_generates_possibilities() {
        let actual: Vec<Vec<usize>> = Combinations::new(5, 3).collect();
        let expected = vec![
            vec![1, 1, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 1, 1],
            vec![0, 1, 1, 0, 1],
            vec![0, 1, 0, 1, 1],
            vec![0, 0, 1, 1, 1],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn combinations_6_4_generates_possibilities() {
        let actual: Vec<Vec<usize>> = Combinations::new(6, 4).collect();
        let expected = vec![
            vec![1, 1, 1, 0, 0, 1],
            vec![1, 1, 0, 1, 0, 1],
            vec![1, 1, 0, 0, 1, 1],
            vec![1, 0, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 1],
            vec![1, 0, 0, 1, 1, 1],
            vec![0, 1, 1, 1, 0, 1],
            vec![0, 1, 1, 0, 1, 1],
            vec![0, 1, 0, 1, 1, 1],
            vec![0, 0, 1, 1, 1, 1],
        ];

        assert_eq!(actual, expected);
    }
}
