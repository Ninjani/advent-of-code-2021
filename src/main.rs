use color_eyre::Result;
mod day1;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    println!("{}", day1::day1_part1(&input)?);
    println!("{}", day1::day1_part2(&input)?);
    Ok(())
}
