use std::fmt::{Display, Formatter};

pub trait Solver {
    fn solve(&self, input: &str) -> anyhow::Result<Answer>;
}

pub struct Answer {
    pub year: u16,
    pub day: u16,
    pub results: Vec<String>,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "=== YEAR {year}, DAY {day} ===",
            year = self.year,
            day = self.day
        )?;

        for (i, result) in self.results.iter().enumerate() {
            writeln!(f, "Part {index}: {result}", index = i + 1)?;
        }

        Ok(())
    }
}

macro_rules! puzzle_solver_selector {
    ($($idx:literal = $puzzle:expr),*$(,)?) => (
        pub fn select_solver(day: u16) -> Option<Box<dyn Solver>> {
            let solver: Box<dyn Solver> = match day {
                $($idx => Box::new($puzzle),)+
                _ => return None,
            };
            Some(solver)
        }
    )
}

pub(crate) use puzzle_solver_selector;

macro_rules! puzzle_solver {
    ([$year:literal, $day:literal] = $body:tt) => (
        pub struct Puzzle;

        type Answer = $crate::puzzle::Answer;

        impl $crate::puzzle::Solver for Puzzle $body

        impl Puzzle {
            fn answer(results: Vec<String>) -> Answer {
                Answer {
                    year: $year,
                    day: $day,
                    results,
                }
            }
        }

        #[cfg(test)]
        mod file_tests {
            $crate::puzzle::puzzle_test_file_input!($year, $day);
        }
    )
}

pub(crate) use puzzle_solver;

macro_rules! answer {
    ($($result:expr),*) => (
        return Ok(Puzzle::answer(vec![
            $($result.to_string()),*
        ]))
    )
}

pub(crate) use answer;

#[cfg(test)]
macro_rules! puzzle_test_file_input {
    ($year:literal, $day:literal) => {
        #[test]
        #[cfg_attr(not(feature = "test_file_input"), ignore)]
        fn test_file_input() {
            use super::*;
            use std::fs;
            use std::io::Read;
            use $crate::puzzle::Solver;

            let path = format!("test_data/{year}/{day:0>2}", year = $year, day = $day);
            let input_path = format!("{path}/input.txt");
            let solution_path = format!("{path}/solution.txt");

            let mut input_file = fs::File::open(input_path).unwrap();
            let mut solution_file = fs::File::open(solution_path).unwrap();

            let mut input = String::new();
            input_file.read_to_string(&mut input).unwrap();

            let mut expected_solution = String::new();
            solution_file
                .read_to_string(&mut expected_solution)
                .unwrap();

            let answer = Puzzle.solve(&input);

            assert!(answer.is_ok());

            let solution = answer.unwrap();

            for (idx, expected) in expected_solution.lines().enumerate() {
                let actual = solution.results.get(idx);
                assert!(
                    actual.is_some(),
                    "no result for part {} of the puzzle",
                    idx + 1
                );
                let actual = actual.unwrap();
                assert_eq!(actual, expected, "expected {expected}, got {actual}");
            }
        }
    };
}

#[cfg(test)]
pub(crate) use puzzle_test_file_input;
