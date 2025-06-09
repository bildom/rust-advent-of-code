use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_03::helpers::*;

mod helpers;

puzzle_solver!(
    [2015, 3] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<Answer> {
            let mut first_year = FirstYear::default();
            let mut second_year = SecondYear::default();

            for (i, c) in input.chars().enumerate() {
                let movement = Movement::from_char(c)?;

                first_year.move_and_add(movement);
                second_year.move_and_add(movement, i % 2);
            }

            let first_year = first_year.count() as u32;
            let second_year = second_year.count() as u32;

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
    #[case(">", 2, 2)]
    #[case("^>v<", 4, 3)]
    #[case("^v", 2, 3)]
    #[case("^v^v^v^v^v", 2, 11)]
    #[case("", 1, 1)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_first_year: u32,
        #[case] expected_second_year: u32,
    ) {
        let answer = Puzzle.solve(input);

        assert!(answer.is_ok());

        let answer = answer.unwrap();

        assert_eq!(answer.results[0], expected_first_year.to_string());
        assert_eq!(answer.results[1], expected_second_year.to_string());
    }

    #[rstest]
    #[case(">^v<a")]
    #[case("asdf")]
    fn negative_tests(#[case] input: &str) {
        let solution = Puzzle.solve(input);

        assert!(solution.is_err());
    }
}
