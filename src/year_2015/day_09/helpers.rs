use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;

type LocationIdx = usize;

pub struct TravelPlanner {
    locations: Vec<String>,
    distances: HashMap<(LocationIdx, LocationIdx), u16>,
    min_dist: Option<u16>,
    max_dist: Option<u16>,
}

impl TravelPlanner {
    pub fn get_min_dist(&self) -> Option<u16> {
        self.min_dist
    }

    pub fn get_max_dist(&self) -> Option<u16> {
        self.max_dist
    }

    pub fn build_from(input: &str) -> anyhow::Result<Self> {
        let mut locations = Vec::new();
        let mut distances = HashMap::new();

        let parser = Parser::default();

        for line in input.lines() {
            let distance = parser.parse(line)?;
            let loc_idx_1 = Self::substitute_name_with_index(&distance.location_1, &mut locations);
            let loc_idx_2 = Self::substitute_name_with_index(&distance.location_2, &mut locations);

            distances
                .entry((loc_idx_1, loc_idx_2))
                .or_insert(distance.value);
            distances
                .entry((loc_idx_2, loc_idx_1))
                .or_insert(distance.value);
        }

        Ok(Self {
            locations,
            distances,
            min_dist: None,
            max_dist: None,
        })
    }

    fn substitute_name_with_index(location: &str, dictionary: &mut Vec<String>) -> LocationIdx {
        if let Some(idx) = dictionary.iter().position(|n| n == location) {
            idx
        } else {
            dictionary.push(location.to_string());
            dictionary.len() - 1
        }
    }

    pub fn calculate_distances(&mut self) -> anyhow::Result<()> {
        for permutation in (0..self.locations.len()).permutations(self.locations.len()) {
            let mut sum_distance = 0;

            for pair in permutation.windows(2) {
                let from = pair[0];
                let to = pair[1];

                sum_distance += self
                    .distances
                    .get(&(from, to))
                    .ok_or_else(|| anyhow!("could not find route: {from} to: {to}"))?;
            }

            self.min_dist = match self.min_dist {
                Some(min_dist) => Some(min(min_dist, sum_distance)),
                None => Some(sum_distance),
            };

            self.max_dist = match self.max_dist {
                Some(max_dist) => Some(max(max_dist, sum_distance)),
                None => Some(sum_distance),
            };
        }

        Ok(())
    }
}

struct Distance {
    location_1: String,
    location_2: String,
    value: u16,
}

struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^(?<location1>\S+) to (?<location2>\S+) = (?<distance>\d+)$").unwrap(),
        }
    }
}

impl Parser {
    fn parse(&self, input: &str) -> anyhow::Result<Distance> {
        if let Some(caps) = self.re.captures(input) {
            let location_1 = caps["location1"].to_string();
            let location_2 = caps["location2"].to_string();
            let distance = caps["distance"].parse()?;

            let result = Distance {
                location_1,
                location_2,
                value: distance,
            };

            Ok(result)
        } else {
            anyhow::bail!("invalid input {input}");
        }
    }
}
