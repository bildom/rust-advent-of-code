use std::cmp::min;

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

    pub fn animate(&mut self, count: usize) {
        let mut neighbours = vec![vec![0; self.height]; self.width];

        for _ in 0..count {
            for (x, row) in neighbours.iter_mut().enumerate() {
                for (y, neighbours_count) in row.iter_mut().enumerate() {
                    *neighbours_count = self.count_neighbours(x, y);
                }
            }

            for (x, row) in self.array.iter_mut().enumerate() {
                for (y, light) in row.iter_mut().enumerate() {
                    if self.lights_stuck_on.contains(&(x, y)) {
                        continue;
                    }

                    if *light == LightState::On && !(neighbours[x][y] == 2 || neighbours[x][y] == 3)
                    {
                        *light = LightState::Off;
                    } else if *light == LightState::Off && neighbours[x][y] == 3 {
                        *light = LightState::On;
                    }
                }
            }
        }
    }

    fn count_neighbours(&self, center_x: usize, center_y: usize) -> usize {
        let mut count = 0;

        for x in center_x.saturating_sub(1)..=min(self.width - 1, center_x + 1) {
            for y in center_y.saturating_sub(1)..=min(self.height - 1, center_y + 1) {
                if x == center_x && y == center_y {
                    continue;
                } else if self.array[x][y] == LightState::On {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn count_lit(&self) -> usize {
        self.array
            .iter()
            .map(|row| row.iter().filter(|light| **light == LightState::On).count())
            .sum()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LightState {
    On,
    Off,
}
