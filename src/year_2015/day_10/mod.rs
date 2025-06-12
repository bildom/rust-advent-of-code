use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_10::helpers::LookAndSay;

mod helpers;

puzzle_solver!(
    [2015, 10] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let after_40 = LookAndSay::run(input, 40);
            let after_50 = LookAndSay::run(&after_40, 10);

            let length_after_40 = after_40.len();
            let length_after_50 = after_50.len();

            answer!(length_after_40, length_after_50);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("1", 82350, 1166642)]
    #[case("22", 2, 2)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_len_after_40: usize,
        #[case] expected_len_after_50: usize,
    ) {
        let solution = Puzzle.solve(input).unwrap();

        assert_eq!(solution.results[0], expected_len_after_40.to_string());
        assert_eq!(solution.results[1], expected_len_after_50.to_string());
    }
}
