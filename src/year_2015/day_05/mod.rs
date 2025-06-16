use crate::puzzle::{answer, puzzle_solver};
use helpers::{FirstYearCriteria, NiceStringValidator, SecondYearCriteria};

mod helpers;

puzzle_solver!(
    [2015, 5] = {
        fn solve(&self, input: &str) -> anyhow::Result<Answer> {
            let mut first_year = 0u32;
            let mut second_year = 0u32;

            for line in input.lines() {
                if NiceStringValidator::validate::<FirstYearCriteria>(line) {
                    first_year += 1;
                }

                if NiceStringValidator::validate::<SecondYearCriteria>(line) {
                    second_year += 1;
                }
            }

            answer!(first_year, second_year);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use rstest::rstest;

    #[rstest]
    // Part 1
    #[case("ugknbfddgicrmopn", 1, 0)]
    #[case("aaa", 1, 0)]
    #[case("jchzalrnumimnmhp", 0, 0)]
    #[case("haegwjzuvuyypxyu", 0, 0)]
    #[case("dvszwmarrgswjxmb", 0, 0)]
    // Part 2
    #[case("qjhvhtzxzqqjkmpb", 0, 1)]
    #[case("xxyxx", 0, 1)]
    #[case("uurcxstgmygtbstg", 0, 0)]
    #[case("ieodomkazucvgmuy", 0, 0)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_first_year: u32,
        #[case] expected_second_year: u32,
    ) {
        let answer = Puzzle.solve(input).unwrap();

        assert_eq!(answer.results[0], expected_first_year.to_string());
        assert_eq!(answer.results[1], expected_second_year.to_string());
    }
}
