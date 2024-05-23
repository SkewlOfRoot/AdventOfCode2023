use std::collections::HashMap;
use std::io;

use crate::utils::file_utils::read_lines_from_file;

pub fn run() -> io::Result<()> {
    part_one()?;

    Ok(())
}

fn part_one() -> Result<(), io::Error> {
    let lines = read_lines_from_file(r".\src\puzzle_day_3\data")?;

    let mut flagged_numbers: Vec<u32> = Vec::new();

    for i in 0..lines.len() {
        let current_line = lines[i].clone();
        let mut line_above = String::new();
        let mut line_below = String::new();

        if i > 0 {
            line_above = lines[i - 1].clone();
        }
        if i < lines.len() - 1 {
            line_below = lines[i + 1].clone();
        }

        let s = Slice::new(current_line, line_above, line_below);
        for number in s.get_flagged_numbers() {
            flagged_numbers.push(number);
        }
    }
    let sum: u32 = flagged_numbers.iter().sum();
    //println!("{:#?}", flagged_numbers);
    println!("Day 3 part 1 answer: {}", sum);
    Ok(())
}

#[derive(Debug)]
struct Slice {
    line_main: String,
    line_above: String,
    line_below: String,
    map: HashMap<usize, u32>,
}

impl Slice {
    fn new(line: String, line_above: String, line_below: String) -> Slice {
        let mut s = Slice {
            line_main: line,
            line_above: line_above,
            line_below: line_below,
            map: HashMap::new(),
        };
        s.map_numbers();
        s
    }

    fn map_numbers(&mut self) {
        let mut number_string = String::new();
        let mut number_start_index: usize = 0;

        for c in self.line_main.char_indices() {
            if c.1.is_numeric() {
                if number_string.len() == 0 {
                    number_start_index = c.0;
                }
                number_string.push(c.1);

                if number_start_index + number_string.len() == self.line_main.len() {
                    self.map
                        .insert(number_start_index, number_string.parse().unwrap());
                }
            } else {
                if number_string.len() > 0 {
                    self.map
                        .insert(number_start_index, number_string.parse().unwrap());
                }
                number_string.clear();
            }
        }
    }

    fn get_flagged_numbers(&self) -> Vec<u32> {
        let mut flagged_numbers: Vec<u32> = Vec::new();

        let is_symbol_within_index = |text: &str, indexes: Vec<usize>| -> bool {
            let symbols: [char; 10] = ['*', '+', '#', '$', '@', '/', '=', '%', '-', '&'];
            for index in indexes {
                let found_char = text.char_indices().find(|x| x.0 == index);
                if let Some(value) = found_char {
                    if symbols.contains(&value.1) {
                        return true;
                    }
                    continue;
                }
                continue;
            }
            return false;
        };

        for n in self.map.clone() {
            let number_length = n.1.to_string().len();
            let mut flagged: bool = false;
            let min_range = if n.0 == 0 { 0 } else { n.0 - 1 };
            let max_range = if n.0 + number_length == self.line_main.len() {
                n.0 + number_length
            } else {
                n.0 + number_length + 1
            };
            let index_range = min_range..max_range;

            if is_symbol_within_index(self.line_main.as_str(), index_range.clone().collect()) {
                flagged = true;
            } else if self.line_above.len() > 0
                && is_symbol_within_index(&self.line_above, index_range.clone().collect())
            {
                flagged = true;
            } else if self.line_below.len() > 0
                && is_symbol_within_index(&self.line_below, index_range.clone().collect())
            {
                flagged = true;
            }
            // test test test
            /*
            if !flagged {
                if self.line_above.len() > 0 {
                    for i in index_range.clone() {
                        //println!("yo!: {}", n.1);
                        let found_char = self.line_above.char_indices().find(|x| x.0 == i).unwrap();
                        print!("{}", found_char.1);
                    }
                    println!();
                }

                for i in index_range.clone() {
                    let found_char = self.line_main.char_indices().find(|x| x.0 == i).unwrap();
                    print!("{}", found_char.1);
                }
                println!();

                if self.line_below.len() > 0 {
                    for i in index_range.clone() {
                        let found_char = self.line_below.char_indices().find(|x| x.0 == i).unwrap();
                        print!("{}", found_char.1);
                    }
                    println!();
                }
                println!();
            }
            */
            // test test test

            if flagged {
                flagged_numbers.push(n.1)
            }
        }
        flagged_numbers
    }
}
