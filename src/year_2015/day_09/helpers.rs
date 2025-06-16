use crate::dictionary::{Dictionary, DictionaryIdx};
use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;

pub struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^(?<location1>\w+) to (?<location2>\w+) = (?<distance>\d+)$").unwrap(),
        }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<Distance> {
        let result = match self.re.captures(input) {
            Some(caps) => {
                let location_1 = caps["location1"].to_string();
                let location_2 = caps["location2"].to_string();
                let distance = caps["distance"].parse()?;

                Distance {
                    location_1,
                    location_2,
                    value: distance,
                }
            }
            None => anyhow::bail!("invalid input '{input}'"),
        };

        Ok(result)
    }
}

pub struct Distance {
    location_1: String,
    location_2: String,
    value: u16,
}

type LocationIdx = DictionaryIdx;

#[derive(Default)]
pub struct TravelPlanner {
    locations: Dictionary,
    distances: HashMap<(LocationIdx, LocationIdx), u16>,
}

impl TravelPlanner {
    pub fn add(&mut self, distance: Distance) {
        let loc_idx_1 = self.locations.put(&distance.location_1);
        let loc_idx_2 = self.locations.put(&distance.location_2);

        self.distances
            .entry((loc_idx_1, loc_idx_2))
            .or_insert(distance.value);

        self.distances
            .entry((loc_idx_2, loc_idx_1))
            .or_insert(distance.value);
    }

    pub fn calculate_distances(&mut self) -> anyhow::Result<Solution> {
        let mut solution = Solution::default();

        for permutation in (0..self.locations.len()).permutations(self.locations.len()) {
            let mut sum_distance = 0;

            for pair in permutation.windows(2) {
                let from = pair[0];
                let to = pair[1];

                sum_distance += self.distances.get(&(from, to)).ok_or_else(|| {
                    anyhow!(
                        "could not find route from {loc_1} to {loc_2}",
                        loc_1 = self.locations.map_to_name(from),
                        loc_2 = self.locations.map_to_name(to)
                    )
                })?;
            }

            solution.solve_min_distance(sum_distance);
            solution.solve_max_distance(sum_distance);
        }

        Ok(solution)
    }
}

#[derive(Default)]
pub struct Solution {
    pub min_dist: Option<u16>,
    pub max_dist: Option<u16>,
}

impl Solution {
    fn solve_min_distance(&mut self, distance: u16) {
        self.min_dist = match self.min_dist {
            Some(min_dist) => Some(min(min_dist, distance)),
            None => Some(distance),
        };
    }

    fn solve_max_distance(&mut self, distance: u16) {
        self.max_dist = match self.max_dist {
            Some(max_dist) => Some(max(max_dist, distance)),
            None => Some(distance),
        };
    }
}
