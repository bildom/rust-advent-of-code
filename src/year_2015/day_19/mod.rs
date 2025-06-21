use crate::puzzle::{answer, puzzle_solver};
use helpers::{MoleculeReplacer, Parser, ParsingMode};
use show_option::ShowOption;

mod helpers;

puzzle_solver!(
    [2015, 19] = {
        fn solve(&self, input: &str) -> anyhow::Result<Answer> {
            let parser = Parser::default();
            let mut parsing_mode = ParsingMode::Replacements;

            let mut molecule_replacer = MoleculeReplacer::default();

            for line in input.lines() {
                if line.is_empty() {
                    parsing_mode = ParsingMode::Molecule;
                    continue;
                }

                match parsing_mode {
                    ParsingMode::Replacements => {
                        let replacement = parser.parse(line)?;
                        molecule_replacer.add_replacement(replacement);
                    }
                    ParsingMode::Molecule => {
                        molecule_replacer.set_molecule(line);
                        parsing_mode = ParsingMode::Finished;
                    }
                    ParsingMode::Finished => anyhow::bail!("unexpected input: '{line}'"),
                }
            }

            let molecule_count = molecule_replacer.get_molecules_count()?;

            let min_synthesis_steps = molecule_replacer
                .count_synthesis_steps()?
                .show_or("none")
                .to_string();

            answer!(molecule_count, min_synthesis_steps);
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
        "e => H
        e => O
        H => HO
        H => OH
        O => HH

        HOHOHO"
    };

    #[rstest]
    #[case(INPUT, 7, 6)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_molecule_count: u32,
        #[case] expected_min_synthesis_steps: u32,
    ) {
        let solution = Puzzle.solve(input).unwrap();

        assert_eq!(solution.results[0], expected_molecule_count.to_string());
        assert_eq!(
            solution.results[1],
            expected_min_synthesis_steps.to_string()
        );
    }
}
