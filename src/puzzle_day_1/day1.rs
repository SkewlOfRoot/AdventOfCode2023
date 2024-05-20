use std::fs::File;
use std::io::{self, BufRead};

pub fn run() {
    part_one().unwrap();
    part_two().unwrap();
}

fn part_one() -> io::Result<()> {
    let lines = get_lines_from_file()?;
    let numbers: Vec<u32> = get_numbers_from_lines(lines);

    let sum: u32 = numbers.iter().sum();
    println!("Answer part 1: {}", sum);

    Ok(())
}

fn part_two() -> io::Result<()> {
    let lines = get_lines_from_file()?;

    let numbers: Vec<u32> = get_numbers_from_lines_v2(lines);

    let sum: u32 = numbers.iter().sum();
    println!("Answer part 2: {}", sum);

    Ok(())
}

fn get_numbers_from_lines(lines: Vec<String>) -> Vec<u32> {
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
    numbers
}
fn get_numbers_from_lines_v2(lines: Vec<String>) -> Vec<u32> {
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

    let mut numbers: Vec<u32> = Vec::new();

    for line in lines {
        let mut line = line;

        for v in &number_vec {
            line = String::from(&line.replace(v.0, v.1))
        }

        let mut number_string = String::new();
        // Loop through line forwards
        for c in line.chars() {
            if c.is_numeric() {
                number_string.push(c);
                break;
            }
        }

        // // Loop through line backwards
        for c in line.chars().rev() {
            if c.is_numeric() {
                number_string.push(c);
                break;
            }
        }

        if number_string.len() > 0 {
            let number = number_string.parse().unwrap();
            numbers.push(number);
        }
    }
    numbers
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
