use crate::period;
use crate::period::detect_cycle;
use std::collections::BTreeSet as Set;
use std::fmt::Display;

#[derive(Debug)]
pub struct Character {
    info: period::Info,
    modulus: usize,
    unique: Set<usize>,
    repeating: Set<usize>,
}

impl Character {
    pub fn new(
        info: period::Info,
        modulus: usize,
        unique: Set<usize>,
        repeating: Set<usize>,
    ) -> Self {
        Self {
            info,
            modulus,
            unique,
            repeating,
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {:?} {:?}",
            self.info, self.modulus, self.unique, self.repeating
        )
    }
}

pub fn determine_character(seq: &[usize]) -> Option<Character> {
    let differences: Vec<usize> = seq.windows(2).map(|t| t[1] - t[0]).collect();

    match detect_cycle(&differences) {
        Some(info) if info.check(&differences) => {
            let modulus: usize = differences[info.pre_period..(info.pre_period + info.period)]
                .iter()
                .sum();
            let mod_seq: Vec<usize> = seq.iter().map(|n| n % modulus).collect();
            let repeating_elements: Set<usize> = mod_seq
                [info.pre_period..(info.pre_period + info.period)]
                .iter()
                .cloned()
                .collect();
            let prefix: Set<usize> = mod_seq[..info.pre_period].iter().cloned().collect();
            let unique_elements: Set<usize> =
                prefix.difference(&repeating_elements).cloned().collect();
            Option::Some(Character::new(
                info,
                modulus,
                unique_elements,
                repeating_elements,
            ))
        }
        _ => Option::None,
    }
}
