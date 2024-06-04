use crate::utils::file_utils::read_lines_from_file;
use std::collections::HashMap;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let lines = read_lines_from_file(r".\src\day_4\data")?;
    let mut cards: Vec<Card> = Vec::new();
    let mut results: Vec<u32> = Vec::new();

    for line in lines {
        let card = Card::new(line);

        let mut result: u32 = 0;

        for _i in 0..card.get_winning_numbers().len() {
            if result == 0 {
                result = 1;
            } else {
                result *= 2;
            }
        }
        results.push(result);
        cards.push(card);
    }

    let sum: u32 = results.iter().sum();
    println!("Part 1 answer: {}", sum);

    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let lines = read_lines_from_file(r".\src\day_4\data")?;
    let mut cards: Vec<Card> = Vec::new();

    // HashMap with card number, number of winning numbers.
    let mut cmap: HashMap<usize, u8> = HashMap::new();

    for (index, line) in lines.iter().enumerate() {
        let card = Card::new(line.to_string());
        let n_winning_numbers: u8 = card.get_winning_numbers().len().try_into().unwrap();
        cmap.insert(index + 1, n_winning_numbers);
        cards.push(card);
    }

    let mut vec: Vec<(usize, u8)> = cmap.iter().map(|(&k, &v)| (k, v)).collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));

    // HashMap with card number, total number of that card.
    let mut rmap: HashMap<usize, u32> = HashMap::new();

    // Instantiate result map with 1 of each card.
    for (index, _c) in &vec {
        rmap.insert(*index, 1);
    }

    for (index, c) in &vec {
        if *c == 0 {
            continue;
        } else {
            let r = rmap.get(index).unwrap();
            for _rcount in 0..*r as usize {
                let min_range: usize = index + 1;
                let max_range: usize = index + 1 + *c as usize;
                for i in min_range..max_range {
                    let res = rmap.entry(i).or_insert(0);
                    *res += 1;
                }
            }
        }
    }

    let sum: u32 = rmap.values().sum();
    println!("Part 2 answer: {}", sum);

    Ok(())
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u8>,
    played_numbers: Vec<u8>,
}

impl Card {
    fn new(line: String) -> Card {
        let l = line.split(':').last().unwrap();
        let card_split: Vec<&str> = l.split('|').map(|x| x.trim()).collect();

        let extract_numbers = |text: &str| -> Vec<u8> {
            text.replace("  ", " ")
                .split(' ')
                .map(|x| x.parse::<u8>().unwrap())
                .collect()
        };

        let winning_numbers: Vec<u8> = extract_numbers(card_split[0]);
        let played_numbers: Vec<u8> = extract_numbers(card_split[1]);

        Card {
            winning_numbers,
            played_numbers,
        }
    }

    fn get_winning_numbers(&self) -> Vec<u8> {
        let result: Vec<u8> = self
            .played_numbers
            .clone()
            .into_iter()
            .filter(|&n| self.winning_numbers.contains(&n))
            .collect();
        result
    }
}
