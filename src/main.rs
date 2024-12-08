mod day;
use day::{d01, d02};

mod util;
use util::lib;

fn main() {
    day1();
    day2();
}

fn day1() {
    // Day 1 part 1
    let input = lib::file_reader("d01.txt");
    let ans = d01::part_1(&input);
    println!("Total diff d1.1: {ans}");

    // Day 1 part 2
    let ans = d01::part_2(&input);
    println!("Similarity score d1.2: {ans}");
}

fn day2() {
    // Day 2 part 1
    let input = lib::file_reader("d02.txt");
    let _input = String::from(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    );
    let ans = d02::part_1(&input);
    println!("Total safe sequences d2.1: {ans}");

    // Day 2 part 2
    let input = lib::file_reader("d02.txt");
    let _input = String::from(
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
    let ans = d02::part_2_2(&input);
    println!("Total safe sequences d2.2: {ans}");
}
