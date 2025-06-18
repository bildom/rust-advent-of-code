use crate::puzzle::{answer, puzzle_solver};
use anyhow::anyhow;
use helpers::{LightArray, LightState};

mod helpers;

puzzle_solver!(
    [2015, 18] = {
        fn solve(&self, input: &str) -> anyhow::Result<Answer> {
            let height = input.lines().count();
            let width = input
                .lines()
                .next()
                .ok_or_else(|| anyhow!("Empty input"))?
                .chars()
                .count();

            let mut lights = LightArray::new(height, width);

            let mut lights_stuck = LightArray::with_lights_stuck_on(
                height,
                width,
                &[
                    (0, 0),
                    (0, height - 1),
                    (width - 1, 0),
                    (width - 1, height - 1),
                ],
            );

            for (y, line) in input.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    let state = match c {
                        '#' => LightState::On,
                        '.' => LightState::Off,
                        other => anyhow::bail!("unknown character {}", other),
                    };
                    lights.set(x, y, state);
                    lights_stuck.set(x, y, state);
                }
            }

            lights.animate(100);
            lights_stuck.animate(100);

            let standard_count = lights.count_lit();
            let fixed_corners_count = lights_stuck.count_lit();

            answer!(standard_count, fixed_corners_count);
        }
    }
);
