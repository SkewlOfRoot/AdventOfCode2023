use std::collections::HashMap;
use std::io;

use crate::utils::file_utils::read_lines_from_file;

pub fn run() -> io::Result<()> {
    let lines = read_lines_from_file(r".\src\puzzle_day_2\data")?;

    let mut limit_map: HashMap<&str, u8> = HashMap::new();
    limit_map.insert("red", 12);
    limit_map.insert("green", 13);
    limit_map.insert("blue", 14);

    let mut valid_game_ids: Vec<u32> = Vec::new();

    for line in &lines {
        valid_game_ids.push(check_for_game(line, &limit_map));
    }

    let sum: u32 = valid_game_ids.iter().sum();
    println!("Day 2 answer: {}", sum);

    Ok(())
}

fn check_for_game(line: &String, limit_map: &HashMap<&str, u8>) -> u32 {
    let id_and_sample_split: Vec<&str> = line.split(':').collect();
    let id = id_and_sample_split[0].replace("Game ", "");
    let samples: Vec<&str> = id_and_sample_split[1].split(';').collect();

    for sample in &samples {
        if !check_sample(sample, limit_map) {
            return 0;
        }
    }

    return id.parse().unwrap();
}

fn check_sample(sample: &&str, limit_map: &HashMap<&str, u8>) -> bool {
    let mut map: HashMap<&str, u8> = HashMap::new();
    let color_samples: Vec<&str> = sample.split(',').collect();
    for color_sample in &color_samples {
        let color_sample = color_sample.trim();
        let values: Vec<&str> = color_sample.split(' ').collect();
        map.insert(values[1], values[0].parse().unwrap());
    }

    for limit in limit_map {
        let played_color = map.get(limit.0).unwrap_or(&0);
        let limit_color = limit.1;
        if played_color > limit_color {
            return false;
        }
    }

    return true;
}
