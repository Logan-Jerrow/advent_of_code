#![allow(unused_variables, dead_code, rust_2018_idioms)]

pub mod passport;

/// Count the number of valid passports.
pub fn count(input: &str) -> anyhow::Result<usize> {
    let raws = input.split("\n\n").collect_vec();

    let passports = raws
        .iter()
        .map(|p| passport::parse_passport(p).finish())
        .filter_ok(|(_,p)| p.is_some())
        .count();
        // .collect_vec();
    Ok(passports)
}

use anyhow::Result;
use itertools::Itertools;
use nom::Finish;
fn main() -> Result<()> {
    let input = include_str!("../input");
    let count = count(input);
    println!("valid passports: {}", count?);
    
    Ok(())
}
