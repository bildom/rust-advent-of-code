use crate::puzzle::{Solver, puzzle_solver_selector};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

puzzle_solver_selector!(
    1 = day_01::Puzzle,
    2 = day_02::Puzzle,
    3 = day_03::Puzzle,
    4 = day_04::Puzzle,
    5 = day_05::Puzzle,
    6 = day_06::Puzzle,
    7 = day_07::Puzzle,
    8 = day_08::Puzzle,
    9 = day_09::Puzzle,
);
