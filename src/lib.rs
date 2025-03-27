#![deny(missing_docs)]

//! The `sequence` library allows one to examine _subsumfree_ sequences.

pub mod combinatorics;
pub mod density;
pub mod tools;

use combinatorics::combination::Sequence as CombinationSequence;
use combinatorics::word::Sequence as ExpressionSequence;

/// Returns an iterator with iterates over a subsumfree sequence.
pub fn sequence(
    initial: Vec<usize>,
    ceiling: usize,
    duplicate: bool,
) -> Box<dyn Iterator<Item = usize>> {
    if duplicate {
        Box::new(ExpressionSequence::with_maximum(initial, ceiling))
    } else {
        Box::new(CombinationSequence::with_maximum(initial, ceiling))
    }
}
