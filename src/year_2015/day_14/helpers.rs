use regex::Regex;
use std::cmp::max;

pub struct Parser {
    re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            re: Regex::new(r"^\w+ can fly (?<speed>\d+) km/s for (?<fly_time>\d+) seconds, but then must rest for (?<rest_time>\d+) seconds.$").unwrap(),
        }
    }
}

impl Parser {
    pub fn parse(&self, input: &str) -> anyhow::Result<Reindeer> {
        let result = match self.re.captures(input) {
            Some(caps) => {
                let speed = caps["speed"].parse()?;
                let fly_time = caps["fly_time"].parse()?;
                let rest_time = caps["rest_time"].parse()?;

                Reindeer {
                    speed,
                    fly_time,
                    rest_time,
                }
            }
            None => anyhow::bail!("could not parse reindeer data"),
        };

        Ok(result)
    }
}

pub struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn get_cycle_time(&self) -> u32 {
        self.fly_time + self.rest_time
    }
}

#[derive(Default)]
pub struct Race {
    reindeer: Vec<Reindeer>,
}

impl Race {
    pub fn add_reindeer(&mut self, data: Reindeer) {
        self.reindeer.push(data);
    }

    pub fn run(&mut self, time: u32) -> anyhow::Result<RunResult> {
        let mut distances = vec![0u32; self.reindeer.len()];
        let mut points = vec![0u32; self.reindeer.len()];

        for current_second in 0..time {
            for (idx, reindeer) in self.reindeer.iter().enumerate() {
                if current_second % reindeer.get_cycle_time() < reindeer.fly_time {
                    distances[idx] += reindeer.speed;
                }
            }

            let mut current_max = None;

            for distance in &distances {
                current_max = match current_max {
                    Some(max_dist) => Some(max(max_dist, distance)),
                    None => Some(distance),
                }
            }

            for (idx, distance) in distances.iter().enumerate() {
                if distance == current_max.unwrap() {
                    points[idx] += 1;
                }
            }
        }

        let winning_distance = distances.iter().max().cloned();
        let winning_points = points.iter().max().cloned();

        let result = RunResult {
            winning_distance,
            winning_points,
        };

        Ok(result)
    }
}

pub struct RunResult {
    pub winning_distance: Option<u32>,
    pub winning_points: Option<u32>,
}
