use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_06::helpers::{LightArray, Parser};

mod helpers;

puzzle_solver!(
    [2015, 6] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<Answer> {
            let parser = Parser::default();

            let mut light_array = LightArray::<bool>::default();
            let mut enhanced_light_array = LightArray::<u16>::default();

            for line in input.lines() {
                let instruction = &parser.parse(line)?;

                light_array.process(instruction);
                enhanced_light_array.process(instruction);
            }

            let lights_lit = light_array.count_lit();
            let lights_brightness = enhanced_light_array.calculate_brightness();

            answer!(lights_lit, lights_brightness);
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Solver;
    use indoc::indoc;
    use rstest::rstest;

    const INPUT_TURN_ON_AND_OFF: &str = indoc! {
        "turn on 0,0 through 999,999
        turn off 499,499 through 500,500"
    };

    const INPUT_TURN_ON_AND_TOGGLE: &str = indoc! {
        "turn on 0,0 through 999,999
        toggle 499,499 through 500,500"
    };

    #[rstest]
    #[case("turn on 0,0 through 999,999", 1_000_000, 1_000_000)]
    #[case("toggle 0,0 through 999,0", 1_000, 2_000)]
    #[case(INPUT_TURN_ON_AND_OFF, 999_996, 999_996)]
    #[case(INPUT_TURN_ON_AND_TOGGLE, 999_996, 1_000_008)]
    fn positive_tests(
        #[case] input: &str,
        #[case] expected_lights_lit: usize,
        #[case] expected_lights_brightness: u32,
    ) {
        let solution = Puzzle.solve(input).unwrap();

        assert_eq!(solution.results[0], expected_lights_lit.to_string());
        assert_eq!(solution.results[1], expected_lights_brightness.to_string());
    }
}
