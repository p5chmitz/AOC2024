use regex::Regex;

pub fn part_1(input: &str) -> i32 {
    tuple_parser(string_parser(&input))
}

// Utility functions
////////////////////

/** Parse input into prefix expression tuples */
fn string_parser(input: &str) -> Vec<&str>  {
    let regex: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<&str> = regex.find_iter(input).map(|mat| mat.as_str()).collect();
    matches
}

/** Parse a list of tuples, calculate products, and sum them */
fn tuple_parser(input: Vec<&str>) -> i32 {
    let regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for e in input {
        for caps in regex.captures_iter(e) {
            let x: i32 = caps[1].parse().expect("Failed to parse x as an integer");
            let y: i32 = caps[2].parse().expect("Failed to parse y as an integer");
            sum += x * y;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /** Tests that the raw input is being parsed into proper prefix expressions */
    fn d03_1_1() {
        let input = String::from(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        let vec = vec!("mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)");
        assert_eq!(string_parser(&input), vec);
    }

    #[test]
    /** Tests that the prefix expressions are being calculated and summed correctly */
    fn d03_1_2() {
        let vec = vec!("mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)");
        assert_eq!(tuple_parser(vec), 161);
    }
}
