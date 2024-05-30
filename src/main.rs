mod puzzle_day_1;
mod puzzle_day_2;
mod puzzle_day_3;
mod puzzle_day_4;
mod utils;

use crate::puzzle_day_1::day1;
use crate::puzzle_day_2::day2;
use crate::puzzle_day_3::day3;
use crate::puzzle_day_4::day4;
use std::io::{self};

fn main() -> io::Result<()> {
    let run_day: u8 = 4;

    match run_day {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        _ => println!("Invalid day."),
    }

    Ok(())
}
