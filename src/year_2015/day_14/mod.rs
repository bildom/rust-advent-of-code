use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_14::helpers::{Parser, Race};
use show_option::ShowOption;

mod helpers;

puzzle_solver!(
    [2015, 14] = {
        fn solve(&self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let parser = Parser::default();
            let mut race = Race::default();

            for line in input.lines() {
                let reindeer = parser.parse(line)?;
                race.add_reindeer(reindeer);
            }

            let run_result = race.run(2503)?;

            let winning_distance = run_result.winning_distance.show_or("none").to_string();
            let winning_points = run_result.winning_points.show_or("none").to_string();

            answer!(winning_distance, winning_points);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use indoc::indoc;
    use rstest::rstest;

    const INPUT: &str = indoc! {
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
    };

    #[rstest]
    #[case(INPUT, 2660, 1564)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_max_distance: u32,
        #[case] expected_winning_points: u32,
    ) {
        let solution = Puzzle.solve(input).unwrap();

        assert_eq!(solution.results[0], expected_max_distance.to_string());
        assert_eq!(solution.results[1], expected_winning_points.to_string());
    }
}
