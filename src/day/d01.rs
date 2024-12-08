pub fn part_1(input: &str) -> isize {
    // 1) Split the input into two unsorted lists
    let mut v1: Vec<isize> = vec![];
    let mut v2: Vec<isize> = vec![];
    for e in input.lines() {
        let mut split = e.split_whitespace();
        v1.push(split.next().unwrap().parse().ok().unwrap());
        v2.push(split.next().unwrap().parse().ok().unwrap());
    }
    // 2) Sort em, Johnny!
    v1.sort();
    v2.sort();
    // 3) Calculate the individual differences and push to a 3rd list
    let mut diff = 0;
    for (i, val) in v1.iter().enumerate() {
        diff += (val - v2[i]).abs() // Pushes absolute value of signed int diffs
    }
    diff
}

pub fn part_2(input: &str) -> isize {
    // 1) Split the input into two unsorted lists
    let mut v1: Vec<isize> = vec![];
    let mut v2: Vec<isize> = vec![];
    for e in input.lines() {
        let mut split = e.split_whitespace();
        v1.push(split.next().unwrap().parse().ok().unwrap());
        v2.push(split.next().unwrap().parse().ok().unwrap());
    }
    // 2) Compare the lists in O(n^2) time for frequency of appearance as similiarity score
    let mut sim_score = 0;
    for (_, v1val) in v1.iter().enumerate() {
        let mut counter = 0;
        for (v2i, _) in v2.iter().enumerate() {
            if *v1val == v2[v2i] {
                counter += 1
            }
        }
        sim_score += counter * *v1val
    }
    sim_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d01_1() {
        let input = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(part_1(&input), 11);
    }

    #[test]
    /** Who knew they'd re-use the same test data? */
    fn d01_2() {
        let input = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(part_2(&input), 31);
    }
}
