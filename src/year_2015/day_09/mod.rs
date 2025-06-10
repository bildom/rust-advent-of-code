use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_09::helpers::TravelPlanner;

mod helpers;

puzzle_solver!(
    [2015, 9] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let mut planner = TravelPlanner::build_from(input)?;

            planner.calculate_distances()?;

            let min_distance = planner.get_min_dist().unwrap();
            let max_distance = planner.get_max_dist().unwrap();

            answer!(min_distance, max_distance);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;

    #[test]
    fn positive_test() {
        let input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

        let solution = Puzzle.solve(input);

        assert!(solution.is_ok());

        let solution = solution.unwrap();

        assert_eq!(solution.results[0], "605");
        assert_eq!(solution.results[1], "982");
    }
}
