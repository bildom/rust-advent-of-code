use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_04::helpers::*;

mod helpers;

puzzle_solver!(
    [2015, 4] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<Answer> {
            let mut hasher = Hasher::default();

            let for_5_zeros = hasher.calculate_suffix(input, 0, Hasher::starts_with_5_zeros)?;

            let for_6_zeros =
                hasher.calculate_suffix(input, for_5_zeros, Hasher::starts_with_6_zeros)?;

            answer!(for_5_zeros, for_6_zeros);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043, 6742839)]
    #[case("pqrstuv", 1048970, 5714438)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_for_5_zeros: u32,
        #[case] expected_for_6_zeros: u32,
    ) {
        let answer = Puzzle.solve(input);

        assert!(answer.is_ok());

        let answer = answer.unwrap();

        assert_eq!(answer.results[0], expected_for_5_zeros.to_string());
        assert_eq!(answer.results[1], expected_for_6_zeros.to_string());
    }
}
