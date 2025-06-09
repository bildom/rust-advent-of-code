use std::fmt::Display;

pub struct Elevator {
    floor: i32,
    instructions_processed: u16,
    basement_index: BasementIndex,
}

impl Default for Elevator {
    fn default() -> Self {
        Elevator {
            floor: 0,
            instructions_processed: 0,
            basement_index: BasementIndex::None,
        }
    }
}

impl Elevator {
    const FLOOR_UP: char = '(';
    const FLOOR_DOWN: char = ')';

    pub fn get_floor(&self) -> i32 {
        self.floor
    }

    pub fn get_basement_index(&self) -> BasementIndex {
        self.basement_index
    }

    pub fn process(&mut self, instruction: char) -> anyhow::Result<()> {
        match instruction {
            Elevator::FLOOR_UP => self.floor += 1,
            Elevator::FLOOR_DOWN => self.floor -= 1,

            other => {
                anyhow::bail!(
                    "invalid character '{other}' (only '{up}' and '{down}' allowed)",
                    up = Elevator::FLOOR_UP,
                    down = Elevator::FLOOR_DOWN,
                );
            }
        }

        self.instructions_processed += 1;

        if self.basement_index.is_none() && self.floor < 0 {
            self.basement_index = BasementIndex::Some(self.instructions_processed);
        }

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum BasementIndex {
    Some(u16),
    None,
}

impl BasementIndex {
    fn is_none(&self) -> bool {
        matches!(*self, BasementIndex::None)
    }
}

impl Display for BasementIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BasementIndex::Some(index) => index.to_string(),
            BasementIndex::None => String::from("none"),
        };
        write!(f, "{}", str)
    }
}
