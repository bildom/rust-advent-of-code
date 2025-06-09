use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_01::helpers::*;

mod helpers;

puzzle_solver!(
    [2015, 1] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<Answer> {
            let mut elevator = Elevator::default();

            for c in input.chars() {
                elevator.process(c)?
            }

            let final_floor = elevator.get_floor();
            let basement_index = elevator.get_basement_index();

            answer!(final_floor, basement_index);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use crate::year_2015::day_01::helpers::BasementIndex;
    use rstest::rstest;

    #[rstest]
    // Part 1
    #[case("(())", 0, BasementIndex::None)]
    #[case("()()", 0, BasementIndex::None)]
    #[case("(((", 3, BasementIndex::None)]
    #[case("(()(()(", 3, BasementIndex::None)]
    #[case("))(((((", 3, BasementIndex::Some(1))]
    #[case("())", -1, BasementIndex::Some(3))]
    #[case("))(", -1, BasementIndex::Some(1))]
    #[case(")))", -3, BasementIndex::Some(1))]
    #[case(")())())", -3, BasementIndex::Some(1))]
    // Part 2
    #[case(")", -1, BasementIndex::Some(1))]
    #[case("()())", -1, BasementIndex::Some(5))]
    // Other
    #[case("", 0, BasementIndex::None)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_final_floor: i32,
        #[case] expected_basement_index: BasementIndex,
    ) {
        let answer = Puzzle.solve(input);

        assert!(answer.is_ok());

        let answer = answer.unwrap();

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
