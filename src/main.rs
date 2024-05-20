mod puzzle_day_1;
mod puzzle_day_2;
mod utils;

use crate::puzzle_day_1::day1;
use crate::puzzle_day_2::day2;
use std::io::{self};

fn main() -> io::Result<()> {
    let run_day: u8 = 2;

    match run_day {
        1 => day1::run()?,
        2 => day2::run()?,
        _ => println!("Invalid day."),
    }

    Ok(())
}
