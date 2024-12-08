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
    let ans = d02::part_1(&input);
    println!("Total safe sequences d2.1: {ans}");

    // Day 2 part 2
    let input = lib::file_reader("d02.txt");
    let ans = d02::part_2(&input);
    println!("Total safe sequences d2.2: {ans}");
}
