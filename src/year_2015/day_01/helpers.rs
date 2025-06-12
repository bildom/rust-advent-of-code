#[derive(Default)]
pub struct Elevator {
    floor: i32,
    instructions_processed: usize,
    basement_index: Option<usize>,
}

impl Elevator {
    pub fn get_floor(&self) -> i32 {
        self.floor
    }

    pub fn get_basement_index(&self) -> Option<usize> {
        self.basement_index
    }

    pub fn process(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.floor += 1,
            Direction::Down => self.floor -= 1,
        }

        self.instructions_processed += 1;

        if self.basement_index.is_none() && self.floor < 0 {
            self.basement_index = Some(self.instructions_processed);
        }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn from(c: char) -> anyhow::Result<Self> {
        let result = match c {
            '(' => Self::Up,
            ')' => Self::Down,

            other => anyhow::bail!("invalid character '{other}'"),
        };

        Ok(result)
    }
}
