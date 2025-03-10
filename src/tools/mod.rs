//! The `tools` module contains various utility data structures and methods
//! that work on subsumfree sequences.

pub mod character;
pub mod expression;
pub mod period;

use std::cmp::Ordering;

/// An `ItemCandidate` keeps track which element is under scrutiny.
#[derive(Debug, PartialEq, Eq)]
pub enum ItemCandidate {
    /// An index into the initial sequence. Certainly part of the sequence.
    Index(usize, Vec<usize>),
    /// A candidate for which it needs to be determined if it is part of the sequence.
    Element(usize),
}

impl ItemCandidate {
    /// Determine the next `ItemCandidate`.
    ///
    /// It there still is a initial segment left, increment the index.
    /// Otherwise, either
    /// * pick the smallest number bigger than the initial segment.
    /// * Or pick the next number after the current candidate.
    pub fn next(&self) -> Self {
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

#[cfg(test)]
mod tests {
    use super::ItemCandidate;

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
}
