mod puzzle_day_1;
mod puzzle_day_2;
mod puzzle_day_3;
mod puzzle_day_4;
mod puzzle_day_5;
mod utils;

use std::error::Error;

use crate::puzzle_day_1::day1;
use crate::puzzle_day_2::day2;
use crate::puzzle_day_3::day3;
use crate::puzzle_day_4::day4;
use crate::puzzle_day_5::day5;

fn main() -> Result<(), Box<dyn Error>> {
    let run_day: u8 = 5;

    match run_day {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        5 => day5::run()?,
        _ => println!("Invalid day."),
    }

    Ok(())
}
