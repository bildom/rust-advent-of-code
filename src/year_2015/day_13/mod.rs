use crate::puzzle::{answer, puzzle_solver};
use helpers::{Parser, SeatingArrangement};
use show_option::ShowOption;

mod helpers;

puzzle_solver!(
    [2015, 13] = {
        fn solve(&self, input: &str) -> anyhow::Result<Answer> {
            let parser = Parser::default();

            let mut arrangement = SeatingArrangement::default();

            for line in input.lines() {
                let relation = parser.parse(line)?;
                arrangement.add_relation(&relation);
            }

            let solution = arrangement.calculate_happiness()?;

            let max_happiness = solution.max_happiness.show_or("none").to_string();
            let max_happiness_with_add =
                solution.max_happiness_with_add.show_or("none").to_string();

            answer!(max_happiness, max_happiness_with_add);
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
        "Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol."
    };

    #[rstest]
    #[case(INPUT, 330, 286)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_max_happiness: i32,
        #[case] expected_max_happiness_with_add: i32,
    ) {
        let solution = Puzzle.solve(input).unwrap();

        assert_eq!(solution.results[0], expected_max_happiness.to_string());
        assert_eq!(
            solution.results[1],
            expected_max_happiness_with_add.to_string()
        );
    }
}
