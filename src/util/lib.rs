use std::fs::File;
use std::io::Read;

// NOTE: Execute file reads from project root (not /src/), 
// otherwise a file not found error may occur.
pub fn file_reader(file: &str) -> String {
    // Reads input from file
    let mut file_contents = String::new();
    match File::open(format!("./input/{}", file)) {
        Ok(mut file) => {
            match file.read_to_string(&mut file_contents) {
                Ok(_) => {
                    //println!("File contents:\n{}", input); 
                }
                Err(err) => { eprintln!("Error reading file: {}", err); }
            }
        }
        Err(err) => { eprintln!("Error opening file: {}", err); }
    }
    file_contents
}
