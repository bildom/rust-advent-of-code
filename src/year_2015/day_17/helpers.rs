use itertools::Itertools;
use std::collections::HashMap;

#[derive(Default)]
pub struct Containers {
    sizes: Vec<u32>,
}

impl Containers {
    pub fn add(&mut self, sizes: u32) {
        self.sizes.push(sizes);
    }

    pub fn sort_desc(&mut self) {
        self.sizes = self.sizes.iter().sorted().rev().copied().collect()
    }
}

pub struct CombinationCounter {
    containers: Containers,
}

impl CombinationCounter {
    pub fn new(containers: Containers) -> Self {
        Self { containers }
    }

    pub fn count(&self, amount: u32) -> Solution {
        let mut solution = Solution::default();

        self.check_combinations(&mut solution, 0, 0, amount);

        solution
    }

    fn check_combinations(
        &self,
        solution: &mut Solution,
        index: usize,
        container_count: u32,
        amount: u32,
    ) {
        let size = self.containers.sizes[index];
        let fits = size <= amount;
        let max_times = if fits { 1 } else { 0 };

        for times in 0..=max_times {
            let amount = amount - (times * size);
            let next_index = index + 1;
            let container_count = container_count + times;

            if amount == 0 {
                *solution
                    .combination_count
                    .entry(container_count)
                    .or_insert(0) += 1;

                continue;
            } else if next_index < self.containers.sizes.len() {
                self.check_combinations(solution, next_index, container_count, amount);
            }
        }
    }
}

#[derive(Default)]
pub struct Solution {
    combination_count: HashMap<u32, u32>,
}

impl Solution {
    pub fn get_combination_count(&self) -> Option<u32> {
        self.combination_count.values().sum1()
    }

    pub fn get_minimal_combination_count(&self) -> Option<u32> {
        let min_key = self.combination_count.keys().min()?;
        self.combination_count.get(min_key).copied()
    }
}
