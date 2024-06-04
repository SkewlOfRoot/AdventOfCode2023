use crate::utils::file_utils::read_lines_from_file;
use rayon::prelude::*;
use std::error::Error;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};

pub fn run() -> Result<(), Box<dyn Error>> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let catalog = load_values()?;
    let mut values: Vec<u64> = Vec::new();

    for seed in catalog.seeds {
        let mut val: u64 = seed;
        for category in &catalog.categories {
            for seed_map in &category.maps {
                if let Some(result) = seed_map.lookup_seed(val) {
                    val = result;
                    break;
                }
            }
        }
        values.push(val);
    }

    let lowest_val = values.iter().min().unwrap();
    println!("Part one answer: {}", lowest_val);

    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let catalog = load_values()?;
    let mut seed_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    let seed_chunks = catalog.seeds.chunks(2);
    for chunk in seed_chunks {
        match chunk {
            [a, b] => {
                seed_ranges.push(*a..=*a + b);
            }
            _ => unreachable!(),
        }
    }

    let mut min_values: Vec<u64> = Vec::new();
    let mut index = 0;

    for range in seed_ranges {
        let mut seeds: Vec<u64> = Vec::new();

        for s in range {
            seeds.push(s);
        }

        let chs = seeds.chunks(1000000);

        for ch in chs {
            let values_c: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
            let values_c_clone = Arc::clone(&values_c);

            ch.par_iter().for_each(|seed| {
                let mut val: u64 = *seed;
                for category in &catalog.categories {
                    for seed_map in &category.maps {
                        if let Some(result) = seed_map.lookup_seed(val) {
                            val = result;
                            break;
                        }
                    }
                }
                let mut ext_list = values_c_clone.lock().unwrap();
                ext_list.push(val);
            });

            let values_c_locked = values_c.lock().unwrap();
            let min_val = values_c_locked.iter().min().unwrap();
            min_values.push(*min_val);
            index += 1;
            println!("{} Processed 1.000.000 seeds.", index);
        }
    }

    let lowest_val = min_values.iter().min().unwrap();
    println!("Part two answer: {}", lowest_val);
    Ok(())
}

fn load_values() -> Result<MapCatalog, Box<dyn Error>> {
    let lines = read_lines_from_file(r".\src\day_5\data")?;

    let mut categories: Vec<MapCategory> = Vec::new();
    let mut category: MapCategory = MapCategory::new(String::from("init"));

    let seeds: Vec<u64> = lines
        .first()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    for (index, line) in lines.iter().enumerate() {
        if index == 0 {
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.contains("map:") {
            if category.name != "init" {
                categories.push(category);
            }
            category = MapCategory::new(line.trim().to_string());
        } else if line.chars().next().unwrap().is_numeric() {
            let vals: Vec<u64> = line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
            let seed_map = SeedMap::new(vals[0], vals[1], vals[2]);
            category.add_seed_map(seed_map);
        }
    }
    categories.push(category);

    let catalog = MapCatalog { seeds, categories };

    Ok(catalog)
}

#[derive(Debug)]
struct SeedMap {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl SeedMap {
    fn new(dest_start: u64, source_start: u64, length: u64) -> SeedMap {
        SeedMap {
            dest_start,
            source_start,
            length,
        }
    }

    fn lookup_seed(&self, seed: u64) -> Option<u64> {
        if seed < self.source_start || (seed > self.source_start + self.length - 1) {
            return None;
        }

        let diff = seed - self.source_start;
        let result = self.dest_start + diff;
        Some(result)
    }
}

#[derive(Debug)]
struct MapCatalog {
    seeds: Vec<u64>,
    categories: Vec<MapCategory>,
}

#[derive(Debug)]
struct MapCategory {
    name: String,
    maps: Vec<SeedMap>,
}

impl MapCategory {
    fn new(name: String) -> MapCategory {
        MapCategory {
            name,
            maps: Vec::new(),
        }
    }

    fn add_seed_map(&mut self, seed_map: SeedMap) {
        self.maps.push(seed_map);
    }
}
