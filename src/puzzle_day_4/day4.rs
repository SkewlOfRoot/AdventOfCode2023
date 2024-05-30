use crate::utils::file_utils::read_lines_from_file;
use std::io;

pub fn run() -> io::Result<()> {
    part_one()?;
    Ok(())
}

fn part_one() -> Result<(), io::Error> {
    let lines = read_lines_from_file(r".\src\puzzle_day_4\data")?;
    let mut cards: Vec<Card> = Vec::new();
    let mut results: Vec<u32> = Vec::new();

    for line in lines {
        let card = Card::new(line);
        //println!("{:?} win: {:?}", card, card.get_winning_numbers());

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
