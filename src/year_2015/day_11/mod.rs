use crate::puzzle::{answer, puzzle_solver};
use helpers::PasswordGenerator;

mod helpers;

puzzle_solver!(
    [2015, 11] = {
        fn solve(&self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let next_password = PasswordGenerator::find_next_valid_password(input)?;
            let another_password = PasswordGenerator::find_next_valid_password(&next_password)?;

            answer!(next_password, another_password);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("abcdefgh", "abcdffaa", "abcdffbb")]
    #[case("ghijklmn", "ghjaabcc", "ghjbbcdd")]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_next_password: &str,
        #[case] expected_another_password: &str,
    ) {
        let solution = Puzzle.solve(input);

        assert!(solution.is_ok());

        let solution = solution.unwrap();

        assert_eq!(solution.results[0], expected_next_password);
        assert_eq!(solution.results[1], expected_another_password);
    }
}
