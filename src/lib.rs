pub mod period;

#[derive(Debug)]
enum ItemCandidate {
    Index(usize, usize),
    Element(usize),
}

impl ItemCandidate {
    fn next(&self) -> Self {
        match self {
            ItemCandidate::Index(index, c) if *index < 2 => ItemCandidate::Index(index + 1, *c),
            ItemCandidate::Index(_, c) => ItemCandidate::Element(*c),
            ItemCandidate::Element(c) => ItemCandidate::Element(*c + 1),
        }
    }
}

#[derive(Debug)]
pub struct Sequence {
    current: ItemCandidate,
    elements: Vec<usize>,
}

impl Sequence {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            current: ItemCandidate::Index(0, c + 1),
            elements: vec![a, b, c],
        }
    }
}

impl Iterator for Sequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Option::None;
        while result.is_none() {
            match self.current {
                ItemCandidate::Index(index, _) => {
                    result = Option::Some(self.elements[index]);
                }
                ItemCandidate::Element(c) => {
                    match express(c, &self.elements) {
                        ExpressionResult::Unexpressable => {
                            result = Option::Some(c);
                            self.elements.push(c);
                        }
                        ExpressionResult::Expressable(_) => {
                            // try the next item candidate
                        }
                    }
                }
            }
            self.current = self.current.next()
        }
        result
    }
}

#[derive(Debug)]
pub enum ExpressionResult {
    Unexpressable,
    Expressable(Vec<[usize; 3]>),
}

pub fn express(n: usize, elements: &[usize]) -> ExpressionResult {
    let mut expressions = vec![];
    for i in 0..elements.len() {
        let p = elements[i];
        if p >= n {
            break;
        }
        for j in (i + 1)..elements.len() {
            let q = elements[j];
            if (p + q) >= n {
                break;
            }
            for k in (j + 1)..elements.len() {
                let r = elements[k];
                if (p + q + r) > n {
                    break;
                }
                if (p + q + r) == n {
                    expressions.push([p, q, r])
                }
            }
        }
    }
    if expressions.is_empty() {
        ExpressionResult::Unexpressable
    } else {
        ExpressionResult::Expressable(expressions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_computes_correct_elements() {
        let actual: Vec<usize> = Sequence::new(1, 3, 5).take(10).collect();
        let expected: Vec<usize> = vec![1, 3, 5, 6, 7, 8, 22, 23, 24, 25];

        assert_eq!(actual, expected);
    }
}
