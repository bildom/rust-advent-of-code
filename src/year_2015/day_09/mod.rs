use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_09::helpers::TravelPlanner;
use show_option::ShowOption;

mod helpers;

puzzle_solver!(
    [2015, 9] = {
        fn solve(&self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let mut planner = TravelPlanner::build_from(input)?;

            let solution = planner.calculate_distances()?;

            let min_distance = solution.min_dist.show_or("none").to_string();
            let max_distance = solution.max_dist.show_or("none").to_string();

            answer!(min_distance, max_distance);
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
        "London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141"
    };

    #[rstest]
    #[case("A to B = 10", 10, 10)]
    #[case(INPUT, 605, 982)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_min_distance: u16,
        #[case] expected_max_distance: u16,
    ) {
        let solution = Puzzle.solve(input).unwrap();

        assert_eq!(solution.results[0], expected_min_distance.to_string());
        assert_eq!(solution.results[1], expected_max_distance.to_string());
    }
}
