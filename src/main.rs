use std::error::Error;

use advent_of_code_2022 as aoc;

fn main() -> Result<(), Box<dyn Error>> {
    aoc::day_1::day_1_1()?;
    aoc::day_1::day_1_2()?;
    aoc::day_2::day_2_1()?;
    aoc::day_2::day_2_2()?;

    Ok(())
}
