use crate::puzzle::{answer, puzzle_solver};
use crate::year_2015::day_11::helpers::PasswordGenerator;

mod helpers;

puzzle_solver!(
    [2015, 11] = {
        fn solve(&mut self, input: &str) -> anyhow::Result<crate::puzzle::Answer> {
            let next_password = PasswordGenerator::find_next_valid_password(input)?;
            let another_password = PasswordGenerator::find_next_valid_password(&next_password)?;

            answer!(next_password, another_password);
        }
    }
);

#[cfg(test)]
mod tests {}
