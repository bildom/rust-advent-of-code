use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_10::helpers::LookAndSay;

mod helpers;

puzzle_solver!(
    [2015, 10] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let result_1 = LookAndSay::run(input, 40);
            let result_2 = LookAndSay::run(&result_1, 10);

            let len_40_iter = result_1.len();
            let len_50_iter = result_2.len();

            answer!(len_40_iter, len_50_iter);
        }
    }
);

#[cfg(test)]
mod tests {}
