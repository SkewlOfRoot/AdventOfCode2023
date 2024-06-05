use crate::utils::file_utils::read_lines_from_file;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let races = load_races_part1()?;

    let mut results: Vec<u64> = Vec::new();
    for r in races {
        results.push(r.get_pos_outcomes());
    }

    let product: u64 = results.iter().product();
    println!("Part one answer: {}", product);

    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let race: Race = load_race_part2()?;

    let results: u64 = race.get_pos_outcomes();
    println!("Part two answer: {}", results);

    Ok(())
}

fn load_races_part1() -> Result<Vec<Race>, Box<dyn Error>> {
    let lines = read_lines_from_file(r".\src\day_6\data")?;
    let time_line = lines.first().unwrap();
    let distance_line = lines.last().unwrap();

    let extract = |line: &String| -> Vec<u64> {
        line.split(':')
            .last()
            .unwrap()
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u64>().unwrap())
            .collect()
    };

    let times: Vec<u64> = extract(time_line);
    let distances: Vec<u64> = extract(distance_line);
    let time_dist_sets: Vec<(u64, u64)> = times.into_iter().zip(distances).collect();
    let mut races: Vec<Race> = Vec::new();

    for (time, distance) in time_dist_sets {
        races.push(Race::new(time, distance));
    }
    Ok(races)
}

fn load_race_part2() -> Result<Race, Box<dyn Error>> {
    let lines = read_lines_from_file(r".\src\day_6\data")?;
    let time_line = lines.first().unwrap();
    let distance_line = lines.last().unwrap();

    let extract = |line: &String| -> String {
        line.split(':')
            .last()
            .unwrap()
            .trim()
            .chars()
            .filter(|&x| x != ' ')
            .collect()
    };

    let time_string: String = extract(time_line);
    let time: u64 = time_string.parse::<u64>().unwrap();
    let distance_string: String = extract(distance_line);
    let distance: u64 = distance_string.parse::<u64>().unwrap();

    Ok(Race::new(time, distance))
}

#[derive(Debug)]
struct Race {
    time_limit: u64,
    dist_record: u64,
}

impl Race {
    fn new(time_limit: u64, dist_record: u64) -> Race {
        Race {
            time_limit,
            dist_record,
        }
    }

    fn get_pos_outcomes(self) -> u64 {
        let mut possible_outcomes: u64 = 0;
        for i in 0..=self.time_limit {
            let result = calc_dist_traveled(i, self.time_limit);
            if result > self.dist_record {
                possible_outcomes += 1;
            }
        }
        possible_outcomes
    }
}

fn calc_dist_traveled(cook_time: u64, time_limit: u64) -> u64 {
    let travel_time = time_limit - cook_time;
    cook_time * travel_time
}
