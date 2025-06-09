use anyhow::anyhow;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;

type LocationIdx = usize;
type Distance = u16;

pub struct TravelPlanner {
    locations: Vec<String>,
    distances: HashMap<LocationIdx, HashMap<LocationIdx, Distance>>,
    min_dist: Option<Distance>,
    max_dist: Option<Distance>,
}

impl TravelPlanner {
    pub fn get_min_dist(&self) -> Option<Distance> {
        self.min_dist
    }

    pub fn get_max_dist(&self) -> Option<Distance> {
        self.max_dist
    }

    pub fn build_from(input: &str) -> anyhow::Result<Self> {
        let mut locations = Vec::new();
        let mut distances = HashMap::new();

        let parser = LocationParser::default();

        for line in input.lines() {
            let distance = parser.parse(line)?;
            let loc_idx_1 = Self::substitute_name_with_index(&distance.loc_1, &mut locations);
            let loc_idx_2 = Self::substitute_name_with_index(&distance.loc_2, &mut locations);

            distances
                .entry(loc_idx_1)
                .or_insert(HashMap::new())
                .insert(loc_idx_2, distance.value);
            distances
                .entry(loc_idx_2)
                .or_insert(HashMap::new())
                .insert(loc_idx_1, distance.value);
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
            idx as LocationIdx
        } else {
            dictionary.push(location.to_string());
            dictionary.len() as LocationIdx - 1
        }
    }

    pub fn calculate_distances(&mut self) -> anyhow::Result<()> {
        if self.min_dist.is_none() {
            let mut loc_visited = Vec::new();
            let sum_distance = 0 as Distance;
            for idx in 0..self.locations.len() {
                self.solve_traveling_salesman_problem(idx, &mut loc_visited, sum_distance)?;
            }
        }

        if self.min_dist.is_none() || self.max_dist.is_none() {
            anyhow::bail!("could not find minimum distance");
        }

        Ok(())
    }

    fn solve_traveling_salesman_problem(
        &mut self,
        current_loc: LocationIdx,
        loc_visited: &mut Vec<LocationIdx>,
        sum_distance: Distance,
    ) -> anyhow::Result<()> {
        loc_visited.push(current_loc);

        if loc_visited.len() < self.locations.len() {
            for loc_idx in 0..self.locations.len() as LocationIdx {
                if loc_visited.contains(&loc_idx) {
                    continue;
                }

                let sum_distance = sum_distance
                    + self
                        .distances
                        .get(&current_loc)
                        .ok_or(anyhow!("no route found"))?
                        .get(&loc_idx)
                        .ok_or(anyhow!("no route found"))?;

                self.solve_traveling_salesman_problem(loc_idx, loc_visited, sum_distance)?;
            }
        } else {
            self.min_dist = if let Some(min_distance) = self.min_dist {
                Some(min(min_distance, sum_distance))
            } else {
                Some(sum_distance)
            };

            self.max_dist = if let Some(max_dist) = self.max_dist {
                Some(max(max_dist, sum_distance))
            } else {
                Some(sum_distance)
            }
        }

        loc_visited.pop();

        Ok(())
    }
}

struct LocationDistance {
    loc_1: String,
    loc_2: String,
    value: u16,
}

struct LocationParser {
    re: Regex,
}

impl Default for LocationParser {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^(?<loc1>\S+) to (?<loc2>\S+) = (?<distance>\d+)$").unwrap(),
        }
    }
}

impl LocationParser {
    fn parse(&self, input: &str) -> anyhow::Result<LocationDistance> {
        if let Some(caps) = self.re.captures(input) {
            let loc_1 = caps["loc1"].to_string();
            let loc_2 = caps["loc2"].to_string();
            let distance = caps["distance"].parse::<u16>()?;

            let result = LocationDistance {
                loc_1,
                loc_2,
                value: distance,
            };

            Ok(result)
        } else {
            anyhow::bail!("invalid input {input}");
        }
    }
}
