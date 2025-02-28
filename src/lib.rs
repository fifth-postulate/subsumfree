use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound;

pub mod character;
pub mod period;

#[derive(Debug, PartialEq, Eq)]
enum ItemCandidate {
    Index(usize, Vec<usize>),
    Element(usize),
}

impl ItemCandidate {
    fn next(&self) -> Self {
        match self {
            ItemCandidate::Index(index, initial) if (*index + 1) < initial.len() => {
                ItemCandidate::Index(index + 1, initial.clone())
            }
            ItemCandidate::Index(_, initial) => {
                ItemCandidate::Element(initial[initial.len() - 1] + 1)
            }
            ItemCandidate::Element(n) => ItemCandidate::Element(*n + 1),
        }
    }
}

impl Ord for ItemCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            ItemCandidate::Index(i, _) => match other {
                ItemCandidate::Index(j, _) => i.cmp(j),
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
    elements: BTreeSet<usize>,
    maximum: Option<ItemCandidate>,
}

impl Sequence {
    pub fn new(initial: Vec<usize>) -> Self {
        Sequence::initialize(initial, Option::None)
    }

    pub fn with_maximum(initial: Vec<usize>, maximum: usize) -> Self {
        Sequence::initialize(initial, Option::Some(ItemCandidate::Element(maximum)))
    }

    fn initialize(initial: Vec<usize>, maximum: Option<ItemCandidate>) -> Self {
        let elements: BTreeSet<usize> = initial.iter().cloned().collect();
        Self {
            current: ItemCandidate::Index(0, initial),
            elements,
            maximum,
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
            match &self.current {
                ItemCandidate::Index(index, initial) => {
                    result = Option::Some(initial[*index]);
                }
                ItemCandidate::Element(c) => {
                    match express(*c, &self.elements) {
                        ExpressionResult::Unexpressable => {
                            result = Option::Some(*c);
                            self.elements.insert(*c);
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

pub fn express(n: usize, elements: &BTreeSet<usize>) -> ExpressionResult {
    let mut expressions = vec![];
    for p in elements.range(1..n) {
        for q in elements.range((Bound::Excluded(*p), Bound::Excluded(n))) {
            if p + q >= n {
                break;
            }
            let r = n - p - q;
            if *q < r && elements.contains(&r) {
                expressions.push([*p, *q, r]);
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
        assert!(ItemCandidate::Index(0, vec![1, 3, 5]) < ItemCandidate::Index(1, vec![1, 3, 5]));
        assert!(ItemCandidate::Index(2, vec![1, 3, 5]) > ItemCandidate::Index(1, vec![1, 3, 4]));
        assert!(ItemCandidate::Index(0, vec![1, 3, 5]) < ItemCandidate::Element(51));
        assert!(ItemCandidate::Index(1, vec![1, 3, 5]) < ItemCandidate::Element(51));
        assert!(ItemCandidate::Index(2, vec![1, 3, 5]) < ItemCandidate::Element(51));
        assert!(ItemCandidate::Element(37) < ItemCandidate::Element(51));
        assert!(ItemCandidate::Element(51) > ItemCandidate::Element(37));
    }

    #[test]
    fn sequence_computes_correct_elements() {
        let actual: Vec<usize> = Sequence::new(vec![1, 3, 5]).take(10).collect();
        let expected: Vec<usize> = vec![1, 3, 5, 6, 7, 8, 22, 23, 24, 25];

        assert_eq!(actual, expected);
    }

    #[test]
    fn sequence_with_maximum_computes_correct_elements() {
        let actual: Vec<usize> = Sequence::with_maximum(vec![1, 3, 5], 20).take(10).collect();
        let expected: Vec<usize> = vec![1, 3, 5, 6, 7, 8];

        assert_eq!(actual, expected);
    }
}
