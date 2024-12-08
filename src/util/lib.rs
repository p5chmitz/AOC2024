use std::fs::File;
use std::io::Read;

// NOTE: Execute file reads from project root (not /src/),
// otherwise a file not found error may occur.
pub fn file_reader(file: &str) -> String {
    let mut file_contents = String::new();
    match File::open(format!("./input/{}", file)) {
        Ok(mut file) => match file.read_to_string(&mut file_contents) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error reading file: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
    file_contents
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    // 1) Read each input line into a vec
    let mut lines: Vec<Vec<i32>> = vec![];
    for e in input.lines() {
        let vec: Vec<i32> = e
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        lines.push(vec);
    }
    lines
}

