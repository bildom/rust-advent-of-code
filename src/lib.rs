pub use crate::config::{Args, InputArgs};
use crate::puzzle::Answer;

mod config;
mod puzzle;

mod year_2015;

pub fn process(args: Args) -> anyhow::Result<Answer> {
    let input = args.input.extract()?;

    let solver = match args.year {
        2015 => year_2015::select_solver(args.day),
        _ => None,
    };

    let answer = if let Some(mut solver) = solver {
        solver.solve(&input)?
    } else {
        anyhow::bail!(
            "no solver found for: year {year}, day {day}",
            year = args.year,
            day = args.day
        );
    };

    Ok(answer)
}
