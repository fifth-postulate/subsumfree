use crate::combinatorics::Combinations;
use crate::item::ItemCandidate;
use std::collections::BTreeSet as Set;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Data {
    n: usize,
    weights: Vec<usize>,
    iterator: Combinations,
}

impl Data {
    fn new(t: usize, elements: &[usize]) -> Self {
        let m = elements.len();
        let weights = elements.into_iter().cloned().collect();
        let mut iterator = Combinations::new(m, t);

        let n = iterator
            .next()
            .map(|word| word.iter().zip(&weights).map(|pair| pair.0 * pair.1).sum())
            .unwrap_or(0);
        Self {
            n,
            weights,
            iterator,
        }
    }

    fn progress(self) -> Option<Self> {
        let mut iterator = self.iterator;
        iterator
            .next()
            .map(|word| {
                word.iter()
                    .zip(&self.weights)
                    .map(|pair| pair.0 * pair.1)
                    .sum()
            })
            .map(|n: usize| Data {
                n: n,
                weights: self.weights,
                iterator,
            })
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.n.cmp(&self.n)
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Option::Some(self.cmp(other))
    }
}

pub struct Sequence {
    t: usize,
    current: ItemCandidate,
    elements: Set<usize>,
    ceiling: Option<ItemCandidate>,
    expressions: BinaryHeap<Data>,
}

impl Sequence {
    pub fn new(initial: Vec<usize>) -> Self {
        Self::initialize(initial, Option::None)
    }

    pub fn with_maximum(initial: Vec<usize>, ceiling: usize) -> Self {
        Self::initialize(initial, Option::Some(ItemCandidate::Element(ceiling)))
    }

    fn initialize(initial: Vec<usize>, ceiling: Option<ItemCandidate>) -> Self {
        let elements: Set<usize> = initial.iter().cloned().collect();
        let n = initial.len();
        let mut expressions: BinaryHeap<Data> = BinaryHeap::new();
        expressions.push(Data::new(n, &initial));
        Self {
            t: n,
            current: ItemCandidate::Index(0, initial),
            elements,
            ceiling,
            expressions,
        }
    }

    fn unexpressable(&mut self, c: usize) -> Option<usize> {
        self.elements.insert(c);
        let prefix: Vec<usize> = self.elements.iter().cloned().collect();
        self.expressions.push(Data::new(self.t, &prefix));
        self.current = self.current.next();
        Option::Some(c)
    }

    fn expressable(&mut self) {
        let data = self.expressions.pop().unwrap(/* safe because we peeked */);
        if let Option::Some(next) = data.progress() {
            self.expressions.push(next);
        }
        self.current = self.current.next();
    }

    fn unknown(&mut self) {
        let data = self.expressions.pop().unwrap(/* safe because we peeked */);
        if let Option::Some(next) = data.progress() {
            self.expressions.push(next);
        }
    }
}

impl Iterator for Sequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Option::None;
        while result.is_none()
            && self
                .ceiling
                .as_ref()
                .map(|max| self.current < *max)
                .unwrap_or(true)
        {
            match &self.current {
                ItemCandidate::Index(index, initial) => {
                    result = Option::Some(initial[*index]);
                    self.current = self.current.next();
                }
                ItemCandidate::Element(c) => {
                    match self.expressions.peek() {
                        Option::Some(peek) => {
                            if *c < peek.n {
                                // c is unexpressable
                                result = self.unexpressable(*c);
                            } else if *c == peek.n {
                                self.expressable();
                            } else {
                                self.unknown();
                            }
                        }
                        Option::None => {
                            result = self.unexpressable(*c);
                        }
                    }
                }
            }
        }
        result
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
