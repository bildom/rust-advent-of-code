use std::cmp::min;
use std::ops::RangeInclusive;

pub struct LightArray {
    width: usize,
    height: usize,
    lights_stuck_on: Vec<(usize, usize)>,
    array: Vec<Vec<LightState>>,
}

impl LightArray {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            lights_stuck_on: Vec::new(),
            array: vec![vec![LightState::Off; height]; width],
        }
    }

    pub fn with_lights_stuck_on(
        width: usize,
        height: usize,
        coordinates: &[(usize, usize)],
    ) -> Self {
        let lights_stuck_on = coordinates.to_vec();
        let mut array = vec![vec![LightState::Off; height]; width];

        for (x, y) in lights_stuck_on.iter() {
            array[*y][*x] = LightState::On;
        }

        Self {
            width,
            height,
            lights_stuck_on,
            array,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, state: LightState) {
        if !self.lights_stuck_on.contains(&(x, y)) {
            self.array[x][y] = state;
        }
    }

    pub fn animate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.prepare();

            for col in self.array.iter_mut() {
                for state in col.iter_mut() {
                    *state = state.transition();
                }
            }
        }
    }

    fn prepare(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.lights_stuck_on.contains(&(x, y)) {
                    continue;
                }

                let count = self.count_neighbours(x, y);
                let state = &mut self.array[x][y];

                *state = match *state {
                    LightState::On if !(2..=3).contains(&count) => LightState::TurningOff,
                    LightState::Off if count == 3 => LightState::TurningOn,
                    other => other,
                }
            }
        }
    }

    fn count_neighbours(&self, center_x: usize, center_y: usize) -> usize {
        let mut count = 0;

        for x in Self::get_neighbour_range(center_x, self.width) {
            for y in Self::get_neighbour_range(center_y, self.height) {
                if (x != center_x || y != center_y) && self.array[x][y].is_on() {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_neighbour_range(this: usize, max_idx: usize) -> RangeInclusive<usize> {
        let r_min = this.saturating_sub(1);
        let r_max = min(max_idx - 1, this + 1);

        r_min..=r_max
    }

    pub fn count_lit(&self) -> usize {
        self.array
            .iter()
            .map(|col| col.iter().filter(|light| **light == LightState::On).count())
            .sum()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LightState {
    On,
    Off,
    TurningOff,
    TurningOn,
}

impl LightState {
    fn is_on(&self) -> bool {
        matches!(self, Self::On | Self::TurningOff)
    }

    fn transition(&self) -> Self {
        match self {
            Self::TurningOff => Self::Off,
            Self::TurningOn => Self::On,
            other => *other,
        }
    }
}
