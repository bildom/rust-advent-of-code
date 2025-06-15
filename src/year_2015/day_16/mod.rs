use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_16::helpers::{Analyser, AuntParam, Parser};
use show_option::ShowOption;
use std::collections::HashMap;

mod helpers;

puzzle_solver!(
    [2015, 16] = {
        fn solve(&self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let parser = Parser::default();
            let mut analyser = Analyser::default();

            for line in input.lines() {
                let aunt = parser.parse(line)?;
                analyser.add_aunt(aunt);
            }

            let aunt_to_find = HashMap::from([
                (AuntParam::Children, 3),
                (AuntParam::Cats, 7),
                (AuntParam::Samoyeds, 2),
                (AuntParam::Pomeranians, 3),
                (AuntParam::Akitas, 0),
                (AuntParam::Vizslas, 0),
                (AuntParam::Goldfish, 5),
                (AuntParam::Trees, 3),
                (AuntParam::Cars, 2),
                (AuntParam::Perfumes, 1),
            ]);

            let solution = analyser.solve_aunt_indices(&aunt_to_find)?;

            let index_exact = solution.index_exact.show_or("none").to_string();
            let index_approx = solution.index_approx.show_or("none").to_string();

            answer!(index_exact, index_approx);
        }
    }
);
