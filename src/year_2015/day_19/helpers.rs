use itertools::Itertools;
use rand::prelude::SliceRandom;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

pub enum ParsingMode {
    Replacements,
    Molecule,
    Finished,
}

pub struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^(?<left>\w+) => (?<right>\w+)$").unwrap(),
        }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<Replacement> {
        if let Some(caps) = self.re.captures(input) {
            let left = caps["left"].to_string();
            let right = caps["right"].to_string();

            Ok(Replacement { left, right })
        } else {
            anyhow::bail!("invalid input for replacement `{}`", input);
        }
    }
}

#[derive(Debug)]
pub struct Replacement {
    left: String,
    right: String,
}

#[derive(Default)]
pub struct MoleculeReplacer {
    replacements: Vec<Replacement>,
    molecule: String,
}

#[derive(Error, Debug)]
#[error("couldn't find result")]
struct ErrorCouldntFindResult;

impl MoleculeReplacer {
    pub fn add_replacement(&mut self, replacement: Replacement) {
        self.replacements.push(replacement);
    }

    pub fn set_molecule(&mut self, molecule: &str) {
        self.molecule = molecule.to_owned();
    }

    pub fn get_molecules_count(&self) -> anyhow::Result<usize> {
        let mut molecules = HashSet::new();
        let mut cache = HashMap::new();

        for r in &self.replacements {
            let from = &r.left;
            let to = &r.right;

            let regex = cache.entry(from.as_str()).or_insert(Regex::new(from)?);

            self.replace(&mut molecules, &self.molecule, regex, to);
        }

        Ok(molecules.len())
    }

    pub fn count_synthesis_steps(&self) -> anyhow::Result<Option<u32>> {
        let mut replacements: Vec<&Replacement> = self.replacements.iter().collect_vec();

        let mut cache = HashMap::new();

        for r in replacements.iter() {
            let from = &r.right;
            cache.insert(from.clone(), Regex::new(from)?);
        }

        let mut rand = rand::rng();

        let min_steps = loop {
            let mut breaker = 0;

            replacements.shuffle(&mut rand);

            match self.analyze_molecule(&self.molecule, &replacements, &cache, 0, &mut breaker) {
                Ok(value) => break value,
                Err(ErrorCouldntFindResult) => continue,
            }
        };

        Ok(min_steps)
    }

    fn analyze_molecule(
        &self,
        input: &str,
        replacements: &[&Replacement],
        cache: &HashMap<String, Regex>,
        step: u32,
        breaker: &mut u32,
    ) -> Result<Option<u32>, ErrorCouldntFindResult> {
        let step = step + 1;

        for r in replacements {
            let from = &r.right;
            let replacement = &r.left;

            let regex = cache.get(from).unwrap();

            let mut results = HashSet::new();
            self.replace(&mut results, input, regex, replacement);

            for result in results {
                if result == "e" {
                    return Ok(Some(step));
                }

                *breaker += 1;

                if *breaker > 1000 {
                    return Err(ErrorCouldntFindResult);
                }

                let min_steps =
                    self.analyze_molecule(&result, replacements, cache, step, breaker)?;

                if min_steps.is_some() {
                    return Ok(min_steps);
                }
            }
        }

        Ok(None)
    }

    fn replace(
        &self,
        accumulator: &mut HashSet<String>,
        input: &str,
        regex: &Regex,
        replacement: &str,
    ) {
        for m in regex.find_iter(input) {
            let start = m.start();
            let end = m.end();

            let before = &input[..start];
            let after = &input[end..];

            let result = format!("{before}{replacement}{after}");

            accumulator.insert(result);
        }
    }
}
