//! The density module fascilitates experiments on determining how dense
//! certain k-subsumfree sets are available.
use std::iter::{empty, once};

use crate::combinatorics::Combinations;
use crate::combinatorics::Words;

/// Determines the maximum number of residues in a k-subsumfree depending on
/// a certain modulus.
pub fn maximum(k: usize, modulus: usize) -> (usize, Vec<usize>) {
    let all_residues: Vec<usize> = (0..modulus).into_iter().collect();
    let mut maximum = 0;
    let mut example: Vec<usize> = vec![];
    let mut n = 1;
    while n <= modulus {
        let mut found = false;
        for selected_residues in Combinations::new(modulus, n).map(|characteristic_word| {
            all_residues
                .iter()
                .cloned()
                .zip(characteristic_word)
                .filter(|(_, b)| *b == 1)
                .map(|(n, _)| n)
                .collect::<Vec<usize>>()
        }) {
            if is_subsumfree(k, modulus, &selected_residues) {
                found = true;
                if n > maximum {
                    maximum = n;
                    example = selected_residues.to_vec();
                }
            }
        }
        if found {
            n += 1;
        } else {
            break;
        }
    }
    (maximum, example)
}

/// Determines the maximum number of residues in a k-subsumfree depending on
/// a certain modulus.
pub fn rec_maximum(k: usize, modulus: usize) -> (usize, Vec<usize>) {
    let covered = vec![false; modulus];
    let residues: Vec<usize> = Vec::new();
    let mut maximum: usize = 0;
    let mut example: Vec<usize> = Vec::new();

    extend(k, modulus, 0, covered, residues, &mut maximum, &mut example);

    (maximum, example)
}

fn extend(
    k: usize,
    modulus: usize,
    last_investigated: usize,
    covered: Vec<bool>,
    residues: Vec<usize>,
    maximum: &mut usize,
    example: &mut Vec<usize>,
) {
    let candidates: Vec<usize> = covered
        .iter()
        .enumerate()
        .filter(|(candidate, covered)| !*covered && *candidate > last_investigated)
        .map(|(candidate, _)| candidate)
        .collect();
    if residues.len() + candidates.len() > *maximum {
        // we might have a chance to beat the current record
        if candidates.len() > 0 {
            'candidate_loop: for candidate in candidates {
                let mut new_covered: Vec<bool> = covered.iter().cloned().collect();
                let mut new_residues: Vec<usize> = residues.iter().cloned().collect();
                new_residues.push(candidate);
                for expression in expressions(k, modulus, &new_residues) {
                    if new_residues.contains(&expression) {
                        continue 'candidate_loop;
                    }
                    new_covered[expression] = true;
                }
                extend(
                    k,
                    modulus,
                    candidate,
                    new_covered,
                    new_residues,
                    maximum,
                    example,
                );
            }
        } else {
            // No candidates left, more residues then current record.
            *maximum = residues.len();
            *example = residues;
        }
    }
}

fn expressions<'a>(
    k: usize,
    modulus: usize,
    generators: &'a [usize],
) -> impl Iterator<Item = usize> + 'a {
    let mut iterator: Box<dyn Iterator<Item = usize>> = Box::new(empty::<usize>());
    if generators.len() > 1 {
        for max in 1..=k {
            let gens: Vec<usize> = generators.iter().cloned().collect();
            let sum_iterator = Words::new(generators.len() - 1, k - max)
                .map(move |mut word| {
                    word.push(max);
                    word
                })
                .map(move |word| {
                    word.into_iter()
                        .zip(gens.iter())
                        .map(|(l, r)| l * r)
                        .sum::<usize>()
                        % modulus
                });
            iterator = Box::new(iterator.chain(sum_iterator));
        }
    } else {
        iterator = Box::new(once((k * generators[0]) % modulus));
    }
    iterator
}

fn is_subsumfree(k: usize, modulus: usize, residues: &[usize]) -> bool {
    for word in Words::new(residues.len(), k) {
        let residue = residues
            .iter()
            .zip(&word)
            .map(|(l, r)| l * r)
            .sum::<usize>()
            % modulus;
        if residues.contains(&residue) {
            return false;
        }
    }
    true
}
