use std::fs::File;
use std::io::{self, BufRead};

pub fn run() -> io::Result<()> {
    println!("hello!");

    // let lines = [
    //     "1abc2",
    //     "pqr3stu8vwx",
    //     "a1b2c3d4e5f",
    //     "treb7uchet",
    //     "one",
    //     "x",
    // ];
    let lines = get_lines_from_file()?;

    let mut numbers: Vec<u32> = Vec::new();

    for line in lines {
        let mut number_string = String::new();

        // Loop through line forwards
        for c in line.chars() {
            if c.is_numeric() {
                number_string.push(c);
                break;
            }
        }

        // Loop through line backwards
        for c in line.chars().rev() {
            if c.is_numeric() {
                number_string.push(c);
                break;
            }
        }

        if number_string.len() > 0 {
            numbers.push(number_string.parse().unwrap());
        }
    }

    println!("{:?}", numbers);
    let sum: u32 = numbers.iter().sum();
    println!("Sum is: {}", sum);

    Ok(())
}

fn get_lines_from_file() -> io::Result<Vec<String>> {
    let file_path = r".\src\puzzle_day_1\data";
    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
