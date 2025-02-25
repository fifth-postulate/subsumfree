use std::cmp::Ordering;

pub mod period;

#[derive(Debug, PartialEq, Eq)]
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

impl Ord for ItemCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            ItemCandidate::Index(i, _) => match other {
                ItemCandidate::Index(j, _) => {
                    println!("-->: {} {} {:?}", i, j, i.cmp(j));
                    i.cmp(j)
                }
                ItemCandidate::Element(_) => Ordering::Less,
            },
            ItemCandidate::Element(n) => match other {
                ItemCandidate::Index(_, _) => Ordering::Greater,
                ItemCandidate::Element(m) => n.cmp(m),
            },
        }
    }
}

impl PartialOrd for ItemCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Option::Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Sequence {
    current: ItemCandidate,
    elements: Vec<usize>,
    maximum: Option<ItemCandidate>,
}

impl Sequence {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            current: ItemCandidate::Index(0, c + 1),
            elements: vec![a, b, c],
            maximum: Option::None,
        }
    }

    pub fn with_maximum(a: usize, b: usize, c: usize, maximum: usize) -> Self {
        Self {
            current: ItemCandidate::Index(0, c + 1),
            elements: vec![a, b, c],
            maximum: Option::Some(ItemCandidate::Element(maximum)),
        }
    }
}

impl Iterator for Sequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Option::None;
        while result.is_none()
            && self
                .maximum
                .as_ref()
                .map(|max| self.current < *max)
                .unwrap_or(true)
        {
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
    fn item_candidates_are_ordered() {
        assert!(ItemCandidate::Index(0, 37) < ItemCandidate::Index(1, 51));
        assert!(ItemCandidate::Index(2, 37) > ItemCandidate::Index(1, 51));
        assert!(ItemCandidate::Index(0, 37) < ItemCandidate::Element(51));
        assert!(ItemCandidate::Index(1, 37) < ItemCandidate::Element(51));
        assert!(ItemCandidate::Index(2, 37) < ItemCandidate::Element(51));
        assert!(ItemCandidate::Element(37) < ItemCandidate::Element(51));
        assert!(ItemCandidate::Element(51) > ItemCandidate::Element(37));
    }

    #[test]
    fn sequence_computes_correct_elements() {
        let actual: Vec<usize> = Sequence::new(1, 3, 5).take(10).collect();
        let expected: Vec<usize> = vec![1, 3, 5, 6, 7, 8, 22, 23, 24, 25];

        assert_eq!(actual, expected);
    }

    #[test]
    fn sequence_with_maximum_computes_correct_elements() {
        let actual: Vec<usize> = Sequence::with_maximum(1, 3, 5, 20).take(10).collect();
        let expected: Vec<usize> = vec![1, 3, 5, 6, 7, 8];

        assert_eq!(actual, expected);
    }
}
