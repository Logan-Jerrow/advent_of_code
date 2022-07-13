mod submarine_one;
mod submarine_two;

use crate::command::Commands;
use itertools::Itertools;

/// --- Day 2: Dive! ---
fn main() {
    let input = include_str!("../input.txt");

    println!("--- Day 2: Dive! ---");
    println!("### Part 1 ###");
    let commands = parse_input(input);
    let mut sub = submarine_one::Submarine::default();
    commands.into_iter().for_each(|c| sub.move_sub(c));
    println!("\tThe cross product is: {}", sub.cross()); // Answer: 2073315

    println!("### Part 2 ###");
    let commands = parse_input(input);
    let mut sub = submarine_two::Submarine::default();
    commands.into_iter().for_each(|c| sub.move_sub(c));
    println!("\tThe cross product is: {}", sub.cross()); // Answer:
}

pub fn parse_input(input: &str) -> Vec<Commands> {
    input
        .lines()
        .map(|raw| {
            raw.split_ascii_whitespace()
                .collect_tuple()
                .map(|(c, n)| match c {
                    "forward" => {
                        Commands::Forward(n.parse().expect("Parse error for forward number."))
                    }
                    "down" => Commands::Down(n.parse().expect("Parse error for down number.")),

                    "up" => Commands::Up(n.parse().expect("Parse error for up number.")),
                    _ => panic!("Unknown command"),
                })
        })
        .map(|x| x.unwrap())
        .collect()
}

mod command {
    #[derive(Debug)]
    pub enum Commands {
        Forward(u32),
        Down(u32),
        Up(u32),
    }
}
