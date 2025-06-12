use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_13::helpers::{Parser, SeatingArrangement};

mod helpers;

puzzle_solver!(
    [2015, 13] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let parser = Parser::default();

            let mut arrangement = SeatingArrangement::default();

            for line in input.lines() {
                let relation = parser.parse(line)?;
                arrangement.add_relation(&relation);
            }

            arrangement.calculate_happiness()?;

            let max_happiness = match arrangement.get_max_happiness() {
                Some(max_happiness) => max_happiness,
                None => anyhow::bail!("could not solve arrangement for max happiness"),
            };

            let max_happiness_with_add = match arrangement.get_max_happiness_with_add() {
                Some(max_happiness_with_add) => max_happiness_with_add,
                None => anyhow::bail!(
                    "could not solve arrangement for max happiness with additional (neutral) person"
                ),
            };

            answer!(max_happiness, max_happiness_with_add);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use rstest::rstest;
    use unindent::unindent;

    const INPUT: &str = {
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
    #[case(&unindent(INPUT), 330, 286)]
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
