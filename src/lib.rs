use crate::item::ItemCandidate;
use std::collections::BTreeSet;
use std::ops::Bound;

pub mod character;
pub mod combinatorics;
pub mod item;
pub mod period;
pub mod word;

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
