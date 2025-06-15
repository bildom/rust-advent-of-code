pub use crate::config::{Args, InputArgs};
use crate::puzzle::Answer;

mod config;
mod puzzle;

mod year_2015;

pub fn process(args: Args) -> anyhow::Result<Answer> {
    let year = args.year;
    let day = args.day;

    let input = args.input.extract()?;

    let solver = match year {
        2015 => year_2015::select_solver(day),
        _ => None,
    };

    let answer = match solver {
        Some(solver) => solver.solve(&input)?,
        None => anyhow::bail!("no solver found for: year {year}, day {day}"),
    };

    Ok(answer)
}
