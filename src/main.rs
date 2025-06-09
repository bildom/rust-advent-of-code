use advent_of_code::{Args, process};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let answer = process(Args::parse())?;
    Ok(println!("{}", answer))
}
