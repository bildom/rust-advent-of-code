use crate::puzzle::{answer, puzzle_solver};
use anyhow::Context;
use helpers::{CombinationCounter, Containers};
use show_option::ShowOption;

mod helpers;

puzzle_solver!(
    [2015, 17] = {
        fn solve(&self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let mut containers = Containers::default();

            for line in input.lines() {
                let size = line
                    .parse()
                    .with_context(|| format!("invalid container size: {line}"))?;
                containers.add(size);
            }

            containers.sort_desc();

            let counter = CombinationCounter::new(containers);
            let solution = counter.count(150);

            let combination_count = solution.get_combination_count().show_or("none").to_string();

            let minimal_combination_count = solution
                .get_minimal_combination_count()
                .show_or("none")
                .to_string();

            answer!(combination_count, minimal_combination_count);
        }
    }
);
