use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_02::helpers::Parser;
use anyhow::Context;

mod helpers;

puzzle_solver!(
    [2015, 2] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<Answer> {
            let parser = Parser::default();

            let presents = input
                .lines()
                .map(|line| parser.parse(line))
                .collect::<anyhow::Result<Vec<_>>>()
                .with_context(|| "could not create presents")?;

            let (area, ribbon_length) = presents
                .iter()
                .map(|present| {
                    (
                        present.get_wrapping_paper_area(),
                        present.get_ribbon_length(),
                    )
                })
                .reduce(|(acc_a, acc_rl), (cur_a, cur_rl)| (acc_a + cur_a, acc_rl + cur_rl))
                .with_context(|| "could not calculate solution")?;

            answer!(area, ribbon_length);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", 58, 34)]
    #[case("1x1x10", 43, 14)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_area: u32,
        #[case] expected_ribbon_length: u32,
    ) {
        let answer = Puzzle.solve(input).unwrap();

        assert_eq!(answer.results[0], expected_area.to_string());
        assert_eq!(answer.results[1], expected_ribbon_length.to_string());
    }

    #[rstest]
    #[case("1x1x10x1")]
    #[case("1xAx10")]
    #[case("1x1")]
    #[case("asdf")]
    #[case("")]
    fn negative_tests(#[case] input: &str) {
        let solution = Puzzle.solve(input);

        assert!(solution.is_err());
    }
}
