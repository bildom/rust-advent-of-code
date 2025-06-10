use anyhow::Context;
use regex::Regex;
use std::ops::RangeInclusive;

pub struct InstructionParser {
    pattern: Regex,
}

impl Default for InstructionParser {
    fn default() -> Self {
        InstructionParser {
            pattern: Regex::new(r"^(?<instruction>\D+) (?<from_x>\d+),(?<from_y>\d+) through (?<to_x>\d+),(?<to_y>\d+)$").unwrap()
        }
    }
}

impl InstructionParser {
    pub fn parse(&self, line: &str) -> anyhow::Result<Instruction> {
        let caps = self
            .pattern
            .captures(line)
            .with_context(|| format!("invalid instruction pattern: {line}"))?;

        let from_x = caps["from_x"].parse::<usize>()?;
        let from_y = caps["from_y"].parse::<usize>()?;
        let to_x = caps["to_x"].parse::<usize>()?;
        let to_y = caps["to_y"].parse::<usize>()?;

        let range = Range {
            from_x,
            from_y,
            to_x,
            to_y,
        };

        Instruction::from(&caps["instruction"], range)
    }
}

pub struct Range {
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
}

impl Range {
    fn x_range(&self) -> RangeInclusive<usize> {
        self.from_x..=self.to_x
    }

    fn y_range(&self) -> RangeInclusive<usize> {
        self.from_y..=self.to_y
    }
}

pub enum Instruction {
    TurnOn(Range),
    TurnOff(Range),
    Toggle(Range),
}

impl Instruction {
    fn from(instruction: &str, range: Range) -> anyhow::Result<Self> {
        let instruction = match instruction {
            "turn on" => Self::TurnOn(range),
            "turn off" => Self::TurnOff(range),
            "toggle" => Self::Toggle(range),
            _ => anyhow::bail!("unknown instruction: {instruction}"),
        };

        Ok(instruction)
    }
}

pub struct LightArray<T> {
    array: Vec<Vec<T>>,
}

impl<T: Default + Clone> Default for LightArray<T> {
    fn default() -> Self {
        LightArray {
            array: vec![vec![T::default(); 1000]; 1000],
        }
    }
}

impl<T> LightArray<T> {
    fn process_range<F>(&mut self, range: &Range, instruction: &mut F)
    where
        F: FnMut(&mut T),
    {
        for row in &mut self.array[range.x_range()] {
            for light in &mut row[range.y_range()] {
                instruction(light);
            }
        }
    }
}

impl LightArray<bool> {
    pub fn process(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn(range) => self.process_range(range, &mut |light| *light = true),
            Instruction::TurnOff(range) => self.process_range(range, &mut |light| *light = false),
            Instruction::Toggle(range) => self.process_range(range, &mut |light| *light = !*light),
        }
    }

    pub fn count_lit(&self) -> usize {
        self.array
            .iter()
            .map(|x| x.iter().filter(|x| **x).count())
            .sum()
    }
}

impl LightArray<u16> {
    pub fn process(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn(range) => {
                self.process_range(range, &mut |light| *light = light.saturating_add(1))
            }
            Instruction::TurnOff(range) => {
                self.process_range(range, &mut |light| *light = light.saturating_sub(1))
            }
            Instruction::Toggle(range) => {
                self.process_range(range, &mut |light| *light = light.saturating_add(2))
            }
        }
    }

    pub fn calculate_brightness(&self) -> u32 {
        self.array
            .iter()
            .map(|x| x.iter().filter(|&x| *x > 0).map(|x| *x as u32).sum::<u32>())
            .sum()
    }
}
