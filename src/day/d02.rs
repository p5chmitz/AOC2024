pub fn part_1(input: &str) -> usize {
    // 1) Read each input line into a vec
    let mut lines: Vec<Vec<i32>> = vec![];
    for e in input.lines() {
        let vec: Vec<i32> = e
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        lines.push(vec);
    }
    // 2) Compare values linearly
    let mut unsafe_seq = 0;
    'outer: for e in &lines {
        // 2.1) Check for monotonic increments
        let mut increasing = true;
        let mut decreasing = true;
        for i in 1..e.len() {
            if e[i] < e[i - 1] {
                increasing = false;
            }
            if e[i] > e[i - 1] {
                decreasing = false;
            }
            // Unsafe if the sequence is neither increasing nor decreasing
            if !increasing && !decreasing {
                //println!("Unsafe (monotonic): {:?}", e);
                unsafe_seq += 1;
                continue 'outer; // Proceed to the next sequence
            }
        }
        // 2.2) Check that each sequence progresses by 1, 2, or 3
        for i in 0..e.len() - 1 {
            // Check min & max diff
            let diff = (e[i] - e[i + 1]).abs();
            if diff < 1 || diff > 3 {
                //println!("Unsafe (diff): {:?}", e);
                unsafe_seq += 1;
                continue 'outer; // Proceed to the next sequence
            }
        }
        //println!("Safe: {:?}", e);
    }
    // 3) Return the number of SAFE sequences
    lines.len() - unsafe_seq
}

pub fn part_2_2(input: &str) -> usize {
    // 1) Read each input line into a vec
    let mut lines: Vec<Vec<i32>> = vec![];
    for e in input.lines() {
        let vec: Vec<i32> = e
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        lines.push(vec);
    }

    //let mut unsafe_count = 0;
    let mut safe_count = 0;
    'outer: for e in lines.iter() {
        let mut sins = 0;
        for i in 0..e.len() - 2 {
            // Checks min diff
            if (e[i] - e[i + 1]).abs() < 1 || (e[i] - e[i + 2]).abs() < 1 {
                //print!("mid two");
                sins += 1;
            }
            // Checks min diff for last two elements
            if ((e[e.len() - 1] - e[e.len() - 2]).abs() < 1)
                || ((e[e.len() - 1] - e[e.len() - 3]).abs() < 1)
            {
                //print!("last two");
                sins += 1;
            }
            // Checks max diff
            if (e[i] - e[i + 1]).abs() > 3 || (e[i] - e[i + 2]).abs() > 6 {
                //unsafe_count += 1;
                println!("Unsafe: {:?}", e);
                continue 'outer;
            }
            // Special case for checking the max diff of the last two
            if (e[e.len() - 1] - e[e.len() - 2]).abs() > 3
                || (e[e.len() - 1] - e[e.len() - 3]).abs() > 6
            {
                //unsafe_count += 1;
                println!("Unsafe: {:?}", e);
                continue 'outer;
            }
        }
        if is_dampened_monotonic(e) == false {
            //unsafe_count += 1;
            println!("Unsafe: {:?}", e);
            continue 'outer;
        }
        if sins >= 2 {
            //unsafe_count += 1;
            println!("Unsafe (sins): {:?}", e);
            continue 'outer;
        }
        println!("Safe: {:?}", e);
        safe_count += 1;
    }
    //lines.len() - unsafe_count
    safe_count
}

// Utility functions
////////////////////

fn is_dampened_monotonic(e: &[i32]) -> bool {
    if e.len() <= 2 {
        return true; // Any sequence with 2 or fewer elements is monotonic
    }

    // Checks if a sequence is strictly monotonic
    let is_strictly_monotonic = |e: &[i32]| {
        e.windows(2).all(|pair| pair[0] < pair[1]) || e.windows(2).all(|pair| pair[0] > pair[1])
    };

    // Check the sequence by removing each element one at a time
    for i in 0..e.len() {
        let mut filtered = Vec::with_capacity(e.len() - 1);
        filtered.extend_from_slice(&e[..i]);
        filtered.extend_from_slice(&e[i + 1..]);
        if is_strictly_monotonic(&filtered) {
            return true;
        }
    }

    false // If no single removal makes the sequence strictly monotonic
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
        assert_eq!(part_2_2(&input), 5);
    }
}
