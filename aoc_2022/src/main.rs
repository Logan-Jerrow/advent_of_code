use aoc_2022::*;

const day01: &str = include_str!("../days/day01.txt");

fn main() {
    let elves = process_elves(day01);
    println!("part 1: {0}", most_calories(&elves).unwrap().total());
    println!("part 2: {0}", most_3_total(&elves));
}
