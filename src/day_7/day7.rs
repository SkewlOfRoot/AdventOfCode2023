use crate::utils::file_utils::read_lines_from_file;
use std::collections::HashMap;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let lines = read_lines_from_file(r".\src\day_7\data")?;
    let hands: Vec<Hand> = lines.iter().map(|l| Hand::from(l)).collect();

    let mut hands_map: HashMap<HandStrength, Vec<Hand>> = HashMap::new();
    for h in hands {
        hands_map.entry(h.strength).or_default().push(h.clone());
    }

    let strength_order: Vec<HandStrength> = vec![
        HandStrength::HighCard,
        HandStrength::OnePair,
        HandStrength::TwoPair,
        HandStrength::ThreeOfAKind,
        HandStrength::FullHouse,
        HandStrength::FourOfAKind,
        HandStrength::FiveOfAKind,
    ];

    let mut winnings: Vec<u64> = Vec::new();
    let mut rank: u32 = 1;
    for k in strength_order {
        let mut hands_sorted: Vec<Hand> = hands_map.get(&k).unwrap_or(&Vec::new()).to_vec();
        hands_sorted.sort_by(|a, b| a.sort_val.cmp(&b.sort_val));
        for h in hands_sorted {
            winnings.push((h.bid * rank).into());
            rank += 1;
        }
    }

    let sum: u64 = winnings.iter().sum();
    println!("Part one answer: {}", sum);
    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    println!("Part two answer: {}", 0);
    Ok(())
}

#[derive(Debug, Clone)]
struct Hand {
    bid: u32,
    strength: HandStrength,
    sort_val: String,
}

impl Hand {
    fn from(line: &str) -> Hand {
        let mut split = line.split(' ');
        let cards: String = String::from(split.next().unwrap());
        let bid: u32 = split.last().unwrap().parse::<u32>().unwrap();

        let card_vec: Vec<char> = cards.chars().collect();
        let mut card_map: HashMap<char, u8> = HashMap::new();

        for c in card_vec {
            *card_map.entry(c).or_insert(0) += 1;
        }

        let values: Vec<u8> = card_map.clone().into_values().collect();

        let mut strength = HandStrength::HighCard;
        if values.contains(&5) {
            strength = HandStrength::FiveOfAKind;
        } else if values.contains(&4) {
            strength = HandStrength::FourOfAKind;
        } else if values.contains(&3) && values.contains(&2) {
            strength = HandStrength::FullHouse;
        } else if values.contains(&3) {
            strength = HandStrength::ThreeOfAKind;
        } else if values.iter().filter(|v| **v != 2).collect::<Vec<_>>().len() == 1 {
            strength = HandStrength::TwoPair;
        } else if values.contains(&2) {
            strength = HandStrength::OnePair;
        }

        // A K Q J T 9 8 7 6 5 4 3 2
        // A B C D E F G H I J K L M
        let sort_val_map: HashMap<char, char> = HashMap::from([
            ('A', 'M'),
            ('K', 'L'),
            ('Q', 'K'),
            ('J', 'J'),
            ('T', 'I'),
            ('9', 'H'),
            ('8', 'G'),
            ('7', 'F'),
            ('6', 'E'),
            ('5', 'D'),
            ('4', 'C'),
            ('3', 'B'),
            ('2', 'A'),
        ]);

        let mut sort_val = String::new();
        for c in cards.chars() {
            sort_val.push(*sort_val_map.get(&c).unwrap())
        }

        Hand {
            bid,
            strength,
            sort_val,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum HandStrength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
