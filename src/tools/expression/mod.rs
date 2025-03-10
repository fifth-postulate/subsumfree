//! The `expression` module allows one to determine if, and how, a number
//! is expressable a sum of other elements
//!
//! Currently, can only determine 3-expression. I.e. sum with tree different
//! elements.

//! ## Example
//!
//! ```
//! # use std::collections::BTreeSet;
//! # use sequence::tools::expression::{ExpressionResult, express};
//! let weights : BTreeSet<usize> = vec![1, 3, 5].into_iter().collect();
//! assert_eq!(ExpressionResult::Unexpressable, express(8, &weights));
//! assert_eq!(ExpressionResult::Expressable(vec![[1, 3, 5]]), express(9, &weights));
//! ```

pub use sequence::Sequence;
use std::collections::BTreeSet;
use std::ops::Bound;

mod sequence;

/// The result of an expression request
#[derive(Debug, PartialEq, Eq)]
pub enum ExpressionResult {
    /// A number is not expressable by the weights given.
    Unexpressable,
    /// A number is expressable, with all different ways enumerated.
    Expressable(Vec<[usize; 3]>),
}

/// Express `n` as a 3-sum of the elements.
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
    use super::{ExpressionResult, express};
    use std::collections::BTreeSet;

    #[test]
    fn eight_can_not_be_expressed_in_1_3_5() {
        let weights: BTreeSet<usize> = vec![1, 3, 5].into_iter().collect();
        assert_eq!(ExpressionResult::Unexpressable, express(8, &weights))
    }

    #[test]
    fn nine_can_be_expressed_in_1_3_5() {
        let weights: BTreeSet<usize> = vec![1, 3, 5].into_iter().collect();
        assert_eq!(
            ExpressionResult::Expressable(vec![[1, 3, 5]]),
            express(9, &weights)
        )
    }
}
