mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod utils;

use std::error::Error;

use crate::day_1::day1;
use crate::day_2::day2;
use crate::day_3::day3;
use crate::day_4::day4;
use crate::day_5::day5;
use crate::day_6::day6;
use crate::day_7::day7;

fn main() -> Result<(), Box<dyn Error>> {
    let run_day: u8 = 7;

    match run_day {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        5 => day5::run()?,
        6 => day6::run()?,
        7 => day7::run()?,
        _ => println!("Invalid day."),
    }

    Ok(())
}
