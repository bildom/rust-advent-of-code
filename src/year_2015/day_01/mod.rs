use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_01::helpers::{Direction, Elevator};
use show_option::ShowOption;

mod helpers;

puzzle_solver!(
    [2015, 1] = {
        fn solve(&self, input: &str) -> anyhow::Result<Answer> {
            let mut elevator = Elevator::default();

            for c in input.chars() {
                let direction = Direction::from(c)?;
                elevator.process(direction);
            }

            let final_floor = elevator.get_floor();
            let basement_index = elevator.get_basement_index().show_or("none").to_string();

            answer!(final_floor, basement_index);
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
    #[case("(())", 0, None)]
    #[case("()()", 0, None)]
    #[case("(((", 3, None)]
    #[case("(()(()(", 3, None)]
    #[case("))(((((", 3, Some(1))]
    #[case("())", -1, Some(3))]
    #[case("))(", -1, Some(1))]
    #[case(")))", -3, Some(1))]
    #[case(")())())", -3, Some(1))]
    // Part 2
    #[case(")", -1, Some(1))]
    #[case("()())", -1, Some(5))]
    // Other
    #[case("", 0, None)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_final_floor: i32,
        #[case] expected_basement_index: Option<usize>,
    ) {
        let expected_basement_index = expected_basement_index.show_or("none");

        let answer = Puzzle.solve(input).unwrap();

        assert_eq!(answer.results[0], expected_final_floor.to_string());
        assert_eq!(answer.results[1], expected_basement_index.to_string());
    }

    #[rstest]
    #[case("))a((")]
    #[case("asdf")]
    fn negative_tests(#[case] input: &str) {
        let solution = Puzzle.solve(input);

        assert!(solution.is_err());
    }
}
