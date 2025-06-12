use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

type PersonIdx = usize;

pub struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self { re: Regex::new(r"^(?<person>\S+) would (?<instruction>gain|lose) (?<amount>\d+) happiness units by sitting next to (?<neighbour>\S+).$").unwrap() }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<Relation> {
        if let Some(caps) = self.re.captures(input) {
            let person = caps["person"].to_string();
            let neighbour = caps["neighbour"].to_string();
            let amount = caps["amount"].parse::<i32>()?;
            let instruction = &caps["instruction"];

            let happiness_gain = match instruction {
                "gain" => amount,
                "lose" => -amount,
                other => anyhow::bail!("invalid instruction: {other}"),
            };

            Ok(Relation {
                person,
                neighbour,
                happiness_gain,
            })
        } else {
            anyhow::bail!("could not parse input '{input}'");
        }
    }
}

pub struct Relation {
    person: String,
    neighbour: String,
    happiness_gain: i32,
}

#[derive(Default, Debug)]
pub struct SeatingArrangement {
    people: Vec<String>,
    relations: HashMap<(PersonIdx, PersonIdx), i32>,
    max_happiness: Option<i32>,
    max_happiness_with_add: Option<i32>,
}

impl SeatingArrangement {
    pub fn get_max_happiness(&self) -> Option<i32> {
        self.max_happiness
    }

    pub fn get_max_happiness_with_add(&self) -> Option<i32> {
        self.max_happiness_with_add
    }

    pub fn add_relation(&mut self, relation: &Relation) {
        let person_idx = self.substitute_name_with_index(&relation.person);
        let neighbour_idx = self.substitute_name_with_index(&relation.neighbour);

        let pair = (person_idx, neighbour_idx);

        self.relations
            .entry(pair)
            .or_insert(relation.happiness_gain);
    }

    fn substitute_name_with_index(&mut self, name: &str) -> PersonIdx {
        if let Some(index) = self.people.iter().position(|p| p == name) {
            index
        } else {
            self.people.push(name.to_string());
            self.people.len() - 1
        }
    }

    pub fn calculate_happiness(&mut self) -> anyhow::Result<()> {
        for permutation in (0..self.people.len()).permutations(self.people.len()) {
            let mut sum = 0;
            let mut sum_with_add = 0;

            for (first, second) in permutation.iter().circular_tuple_windows() {
                sum += self.calculate_pair_happiness(*first, *second)?;
            }

            for (first, second) in permutation.iter().tuple_windows() {
                sum_with_add += self.calculate_pair_happiness(*first, *second)?;
            }

            self.max_happiness = match self.max_happiness {
                Some(current_max) => Some(max(current_max, sum)),
                None => Some(sum),
            };

            self.max_happiness_with_add = match self.max_happiness_with_add {
                Some(current_max) => Some(max(current_max, sum_with_add)),
                None => Some(sum_with_add),
            }
        }

        Ok(())
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
