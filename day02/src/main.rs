mod submarine_one;
mod submarine_two;

use itertools::Itertools;

/// --- Day 2: Dive! ---
fn main() {
    let input = include_str!("../input.txt");

    println!("--- Day 2: Dive! ---");
    println!("### Part 1 ###");
    let commands = submarine_one::parse_input(input);
    let mut sub = submarine_one::Submarine::default();
    commands.into_iter().for_each(|c| sub.move_sub(c));
    println!("\tThe cross product is: {}", sub.cross()); // Answer: 2073315
}
