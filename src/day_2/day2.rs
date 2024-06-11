use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::error::Error;

use crate::utils::file_utils::read_lines_from_file;

const DATA_FILE: &str = r".\src\day_2\data";

pub fn run() -> Result<(), Box<dyn Error>> {
    part_one()?;
    part_two()?;

    Ok(())
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let lines = read_lines_from_file(DATA_FILE)?;
    let limit_map: HashMap<&str, u8> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut valid_game_ids: Vec<u32> = Vec::new();

    'outer: for line in &lines {
        let id_and_sample_split: Vec<&str> = line.split(':').collect();
        let id = id_and_sample_split[0].replace("Game ", "");
        let game_samples = get_game_samples(id_and_sample_split[1]);

        for map in game_samples {
            if !check_sample_limit(&limit_map, map) {
                continue 'outer;
            }
        }
        valid_game_ids.push(id.parse().unwrap());
    }

    let sum: u32 = valid_game_ids.iter().sum();
    println!("Day 2 part 1 answer: {}", sum);

    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let lines = read_lines_from_file(DATA_FILE)?;

    let mut products: Vec<u32> = Vec::new();

    for line in lines {
        let id_and_sample_split: Vec<&str> = line.split(':').collect();
        let game_samples = get_game_samples(id_and_sample_split[1]);

        let mut lowest_map: HashMap<&str, u8> = HashMap::new();

        for sample in game_samples {
            for sample_pair in sample {
                match lowest_map.entry(sample_pair.0) {
                    Occupied(mut entry) => {
                        let current_value = entry.get_mut();
                        if *current_value < sample_pair.1 {
                            *current_value = sample_pair.1;
                        }
                    }
                    Vacant(entry) => {
                        entry.insert(sample_pair.1);
                    }
                }
            }
        }

        products.push(lowest_map.values().map(|&num| num as u32).product());
    }

    let sum: u32 = products.iter().sum();
    println!("Day 2 part 2 answer: {}", sum);

    Ok(())
}

fn get_game_samples(samples_string: &str) -> Vec<HashMap<&str, u8>> {
    let samples: Vec<&str> = samples_string.split(';').collect();
    let mut list: Vec<HashMap<&str, u8>> = Vec::new();
    for sample in samples {
        let mut map: HashMap<&str, u8> = HashMap::new();
        let color_samples: Vec<&str> = sample.split(',').collect();
        for color_sample in &color_samples {
            let color_sample = color_sample.trim();
            let values: Vec<&str> = color_sample.split(' ').collect();
            map.insert(values[1], values[0].parse().unwrap());
        }
        list.push(map);
    }
    list
}

fn check_sample_limit(limit_map: &HashMap<&str, u8>, sample_map: HashMap<&str, u8>) -> bool {
    for limit in limit_map {
        let played_color = sample_map.get(limit.0).unwrap_or(&0);
        let limit_color = &limit.1;
        if played_color > limit_color {
            return false;
        }
    }
    true
}
