use crate::tools::period;
use crate::tools::period::detect_cycle;
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

    pub fn write_walnut<T>(&self, name: &str, f: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        for r in &self.repeating {
            writeln!(f, "def is{} \"Ek z=k*{}+{}\":", r, self.modulus, r)?;
        }
        let mut definitions: Vec<String> = Vec::new();
        for unique in &self.unique {
            definitions.push(format!("z={}", unique));
        }
        for repeating in &self.repeating {
            definitions.push(format!("$is{}(z)", repeating));
        }
        let definition = definitions.join(" | ");
        writeln!(f, "\ndef {} \"{}\":", name, definition)?;
        writeln!(
            f,
            "\neval prop_{0} \"Az z>0 => (${0}(z) <=> z>0 & ~(E a,b,c a<b & b<c & ${0}(a) & ${0}(b) & ${0}(c) & a+b+c=z))\"::",
            name
        )
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
