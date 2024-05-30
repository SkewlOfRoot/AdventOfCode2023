use crate::utils::file_utils::read_lines_from_file;
use std::io::{self};

pub fn run() -> io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_one() -> io::Result<()> {
    let lines = read_lines_from_file(r".\src\puzzle_day_1\data")?;
    let numbers: Vec<u8> = get_numbers_from_lines(lines);

    let sum: u32 = numbers.iter().map(|&num| num as u32).sum();
    println!("Answer part 1: {}", sum);

    Ok(())
}

fn part_two() -> io::Result<()> {
    let lines = read_lines_from_file(r".\src\puzzle_day_1\data")?;
    let numbers: Vec<u8> = get_numbers_from_lines_v2(lines);

    let sum: u32 = numbers.iter().map(|&num| num as u32).sum();
    println!("Answer part 2: {}", sum);

    Ok(())
}

fn get_numbers_from_lines(lines: Vec<String>) -> Vec<u8> {
    let mut numbers: Vec<u8> = Vec::new();
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

        if !number_string.is_empty() {
            numbers.push(number_string.parse().unwrap());
        }
    }
    numbers
}
fn get_numbers_from_lines_v2(lines: Vec<String>) -> Vec<u8> {
    let number_vec: Vec<(&str, &str)> = vec![
        ("one", "o1ne"),
        ("two", "t2o"),
        ("three", "thr3e"),
        ("four", "fo4ur"),
        ("five", "fi5e"),
        ("six", "s6x"),
        ("seven", "se7en"),
        ("eight", "eig8ht"),
        ("nine", "n9ne"),
    ];

    let mut replaced_lines: Vec<String> = vec![];
    for line in lines {
        let mut line: String = line;

        for v in &number_vec {
            line = line.replace(v.0, v.1)
        }
        replaced_lines.push(line);
    }

    get_numbers_from_lines(replaced_lines)
}
