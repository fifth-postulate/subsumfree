#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Expressions {
    t: usize,
    current: Option<Vec<usize>>,
}

impl Expressions {
    pub fn new(n: usize, t: usize) -> Self {
        let mut current: Vec<usize> = vec![0; n];
        if n > 0 {
            current[n - 1] = 1;
            current[0] = t - 1
        }
        Self {
            t,
            current: Option::Some(current),
        }
    }
}

impl Iterator for Expressions {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Option::Some(current) => {
                let mut expression: Vec<usize> = current.to_vec();
                let result = Option::Some(expression.to_vec());
                let n = expression.len() - 1;
                let mut index = n;
                if expression[index] < self.t {
                    index -= 1;
                    while index > 0 && expression[index] == 0 {
                        index -= 1;
                    }
                    if expression[index] > 1 || index == n - 1 {
                        expression[index] -= 1;
                        expression[index + 1] += 1;
                    } else {
                        expression[index] = 0;
                        expression[index + 1] = 1;
                        expression.swap(index + 1, n);
                    }
                    self.current = Option::Some(expression);
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
    use super::{Combinations, Expressions};

    #[test]
    fn combinations_4_2_generates_all_possibilities() {
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
    fn combinations_6_4_generates_all_possibilities() {
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

    #[test]
    fn combinatations_3_3_generates_all_possibilities() {
        let actual: Vec<Vec<usize>> = Combinations::new(3, 3).collect();
        let expected = vec![vec![1, 1, 1]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn expressions_4_3_generates_all_possibilities() {
        let actual: Vec<Vec<usize>> = Expressions::new(4, 3).collect();
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
    fn expressions_3_4_generates_all_possibilities() {
        let actual: Vec<Vec<usize>> = Expressions::new(3, 5).collect();
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
