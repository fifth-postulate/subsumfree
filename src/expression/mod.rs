use std::collections::BTreeSet;
use std::ops::Bound;

#[derive(Debug, PartialEq, Eq)]
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
    use super::{ExpressionResult, express};
    use std::collections::BTreeSet;

    #[test]
    fn eight_can_not_be_expressed_in_1_3_5() {
        let weights: BTreeSet<usize> = vec![1, 3, 5].into_iter().collect();
        assert_eq!(ExpressionResult::Unexpressable, express(8, &weights))
    }
}
