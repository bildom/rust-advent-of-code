use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_06::helpers::*;

mod helpers;

puzzle_solver!(
    [2015, 6] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<Answer> {
            let parser = InstructionParser::default();
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
mod tests {}
