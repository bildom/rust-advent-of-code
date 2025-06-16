use crate::puzzle::{answer, puzzle_solver};
use helpers::{Parser, RecipeCalculator};
use show_option::ShowOption;

mod helpers;

puzzle_solver!(
    [2015, 15] = {
        fn solve(&self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let parser = Parser::default();
            let mut calculator = RecipeCalculator::default();

            for line in input.lines() {
                let ingredient = parser.parse(line)?;
                calculator.add_ingredient(ingredient);
            }

            let solution = calculator.solve_recipe(100)?;

            let best_score = solution.best_score.show_or("none").to_string();
            let best_score_500_cal = solution.best_score_500_cal.show_or("none").to_string();

            answer!(best_score, best_score_500_cal);
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
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
    };

    #[rstest]
    #[case(INPUT, 62842880, 57600000)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_best_score: u32,
        #[case] expected_best_score_500_cal: u32,
    ) {
        let solution = Puzzle.solve(input).unwrap();

        assert_eq!(solution.results[0], expected_best_score.to_string());
        assert_eq!(solution.results[1], expected_best_score_500_cal.to_string());
    }
}
