use crate::dictionary::{Dictionary, DictionaryIdx};
use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

pub struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self { re: Regex::new(r"^(?<person>\w+) would (?<instruction>gain|lose) (?<amount>\d+) happiness units by sitting next to (?<neighbour>\w+).$").unwrap() }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<Relation> {
        let result = match self.re.captures(input) {
            Some(caps) => {
                let person = caps["person"].to_string();
                let neighbour = caps["neighbour"].to_string();
                let amount = caps["amount"].parse()?;
                let instruction = &caps["instruction"];

                let happiness_gain: i32 = match instruction {
                    "gain" => amount,
                    "lose" => -amount,
                    other => anyhow::bail!("invalid instruction: {other}"),
                };

                Relation {
                    person,
                    neighbour,
                    happiness_gain,
                }
            }
            None => anyhow::bail!("could not parse input '{input}'"),
        };

        Ok(result)
    }
}

pub struct Relation {
    person: String,
    neighbour: String,
    happiness_gain: i32,
}

type PersonIdx = DictionaryIdx;

#[derive(Default)]
pub struct SeatingArrangement {
    people: Dictionary,
    relations: HashMap<(PersonIdx, PersonIdx), i32>,
}

impl SeatingArrangement {
    pub fn add_relation(&mut self, relation: &Relation) {
        let person_idx = self.people.put(&relation.person);
        let neighbour_idx = self.people.put(&relation.neighbour);

        let pair = (person_idx, neighbour_idx);

        self.relations
            .entry(pair)
            .or_insert(relation.happiness_gain);
    }

    pub fn calculate_happiness(&mut self) -> anyhow::Result<Solution> {
        let mut solution = Solution::default();

        for permutation in (0..self.people.len()).permutations(self.people.len()) {
            let mut sum = 0;
            let mut sum_with_add = 0;

            for (first, second) in permutation.iter().circular_tuple_windows() {
                sum += self.calculate_pair_happiness(*first, *second)?;
            }

            for (first, second) in permutation.iter().tuple_windows() {
                sum_with_add += self.calculate_pair_happiness(*first, *second)?;
            }

            solution.solve_max_happiness(sum);
            solution.solve_max_happiness_with_add(sum_with_add);
        }

        Ok(solution)
    }

    fn calculate_pair_happiness(
        &self,
        person_1: PersonIdx,
        person_2: PersonIdx,
    ) -> anyhow::Result<i32> {
        let result =
            self.get_relation(person_1, person_2)? + self.get_relation(person_2, person_1)?;

        Ok(result)
    }

    fn get_relation(&self, from_idx: PersonIdx, to_idx: PersonIdx) -> anyhow::Result<i32> {
        self.relations
            .get(&(from_idx, to_idx))
            .ok_or_else(|| anyhow!("could not find relation for {from_idx}: {to_idx}"))
            .copied()
    }
}

#[derive(Default)]
pub struct Solution {
    pub max_happiness: Option<i32>,
    pub max_happiness_with_add: Option<i32>,
}

impl Solution {
    fn solve_max_happiness(&mut self, happiness: i32) {
        self.max_happiness = match self.max_happiness {
            Some(current_max) => Some(max(current_max, happiness)),
            None => Some(happiness),
        };
    }

    fn solve_max_happiness_with_add(&mut self, happiness: i32) {
        self.max_happiness_with_add = match self.max_happiness_with_add {
            Some(current_max) => Some(max(current_max, happiness)),
            None => Some(happiness),
        };
    }
}
