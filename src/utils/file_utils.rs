use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
