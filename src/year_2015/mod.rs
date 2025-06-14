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
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

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
    10 = day_10::Puzzle,
    11 = day_11::Puzzle,
    12 = day_12::Puzzle,
    13 = day_13::Puzzle,
    14 = day_14::Puzzle,
    15 = day_15::Puzzle,
);
