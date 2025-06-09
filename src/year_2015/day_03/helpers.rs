use std::collections::HashMap;

pub struct FirstYear {
    counter: PresentCounter,
    coordinates: Coordinates,
}

impl Default for FirstYear {
    fn default() -> Self {
        let mut counter = PresentCounter::default();
        let coordinates = Coordinates::default();

        counter.add(coordinates);

        Self {
            counter,
            coordinates,
        }
    }
}

impl FirstYear {
    pub fn move_and_add(&mut self, movement: Movement) {
        self.coordinates.move_position(movement);
        self.counter.add(self.coordinates);
    }

    pub fn count(&self) -> usize {
        self.counter.count()
    }
}

pub struct SecondYear {
    counter: PresentCounter,
    coordinates: [Coordinates; 2],
}

impl Default for SecondYear {
    fn default() -> Self {
        let mut counter = PresentCounter::default();
        let coordinates = [Coordinates::default(), Coordinates::default()];

        counter.add(coordinates[0]);
        counter.add(coordinates[1]);

        Self {
            counter,
            coordinates,
        }
    }
}

impl SecondYear {
    pub fn move_and_add(&mut self, movement: Movement, index: usize) {
        self.coordinates[index].move_position(movement);
        self.counter.add(self.coordinates[index]);
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
        const LEFT: char = '<';
        const RIGHT: char = '>';
        const UP: char = '^';
        const DOWN: char = 'v';

        let movement = match c {
            LEFT => Movement::Left,
            RIGHT => Movement::Right,
            UP => Movement::Up,
            DOWN => Movement::Down,

            other => {
                anyhow::bail!(
                    "unrecognized character '{other}' (only '{LEFT}', '{RIGHT}', '{UP}' and '{DOWN}' allowed)"
                );
            }
        };

        Ok(movement)
    }
}
