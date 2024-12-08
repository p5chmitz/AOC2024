/***/
pub fn part_1(input: &str) -> usize {
    // 1) Read each input line into a vec
    let lines = parse_input(input);

    // 2) Check each sequence
    let mut safe = 0;
    for vec in lines {
        if is_safe(&vec, None) {
            //println!("\tSafe: {:?}", vec);
            safe += 1;
        } else {
            //println!("\tUnsafe: {:?}", vec);
        }
    }
    safe
}

/***/
pub fn part_2(input: &str) -> usize {
    // 1) Read each input line into a vec
    let lines = parse_input(input);

    // 2) Brute force check eash sequence with a skip value
    let mut safe = 0;
    for vec in lines {
        for i in 0..vec.len() {
            if is_safe(&vec, Some(i)) {
                //println!("\tSafe: {:?}", vec);
                safe += 1;
                break;
            }
            if i == vec.len() - 1 {
                //println!("\tUnsafe: {:?}", vec);
            }
        }
    }
    safe
}

// Utility functions
////////////////////

fn parse_input(input: &str) -> Vec<Vec<i32>> {
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

/** Each sequence is safe if:
 - The sequence is monotonic
 - The diff between values in the sequence is > 0 and < 4 

 NOTE: The skip parameter allows the sequence to be safe
 if the removal of a single index allows the sequence to 
 pass all safety checks */
fn is_safe(input: &Vec<i32>, skip: Option<usize>) -> bool {
    let safe = true;
    let skip = skip.unwrap_or(100); // If None, value is impossibly high
    // 1) Builds a temporary list without the skipped element
    let mut temp: Vec<&i32> = vec![];
    for (i, val) in input.iter().enumerate() {
        if i == skip {
            continue;
        }
        temp.push(val);
    }

    // 2) Checks the sequence for monotonic increments
    let mut increasing = true;
    let mut decreasing = true;
    for i in 1..temp.len() {
        if temp[i] < temp[i - 1] {
            increasing = false;
        }
        if temp[i] > temp[i - 1] {
            decreasing = false;
        }
        // Unsafe if the sequence is neither increasing nor decreasing
        if !increasing && !decreasing {
            return false;
        }
    }

    // 2) Checks that each sequence progresses by 1, 2, or 3
    for i in 0..temp.len() - 1 {
        let diff = (temp[i] - temp[i + 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d02_1() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    /** Who knew they'd re-use the same test data? */
    fn d02_2() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
97 97 98 96 93 90 91
8 11 9 11 10 11
3 4 7 9 12 13 16 16
60 63 60 59 58 55 53 53
25 28 25 22 20 17 13",
        );
        assert_eq!(part_2(&input), 5);
    }
}
