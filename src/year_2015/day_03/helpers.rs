use std::collections::HashMap;

pub struct PresentDelivery {
    counter: PresentCounter,
    deliverers_position: Vec<Coordinates>,
    current_deliverer: usize,
}

impl PresentDelivery {
    pub fn new(deliverers_count: usize) -> Self {
        let mut counter = PresentCounter::default();
        let deliverers_position = vec![Coordinates::default(); deliverers_count];

        for coordinates in deliverers_position.iter() {
            counter.add(*coordinates);
        }

        Self {
            counter,
            deliverers_position,
            current_deliverer: 0,
        }
    }

    pub fn move_and_add(&mut self, movement: Movement) -> anyhow::Result<()> {
        self.deliverers_position
            .get_mut(self.current_deliverer)
            .unwrap()
            .move_position(movement);

        self.counter
            .add(self.deliverers_position[self.current_deliverer]);

        self.current_deliverer = (self.current_deliverer + 1) % self.deliverers_position.len();

        Ok(())
    }

    pub fn count(&self) -> usize {
        self.counter.count()
    }
}

#[derive(Default)]
struct PresentCounter {
    counter: HashMap<Coordinates, u32>,
}

impl PresentCounter {
    fn add(&mut self, coordinates: Coordinates) {
        *self.counter.entry(coordinates).or_insert(0) += 1;
    }

    fn count(&self) -> usize {
        self.counter.len()
    }
}

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn move_position(&mut self, movement: Movement) {
        match movement {
            Movement::Left => self.x -= 1,
            Movement::Right => self.x += 1,
            Movement::Up => self.y += 1,
            Movement::Down => self.y -= 1,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl Movement {
    pub fn from_char(c: char) -> anyhow::Result<Movement> {
        let movement = match c {
            '<' => Movement::Left,
            '>' => Movement::Right,
            '^' => Movement::Up,
            'v' => Movement::Down,

            other => anyhow::bail!("unrecognized character '{other}'"),
        };

        Ok(movement)
    }
}
