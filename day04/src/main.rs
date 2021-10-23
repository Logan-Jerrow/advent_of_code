#![allow(unused_variables, dead_code, rust_2018_idioms)]

pub mod passport;

/// Count the number of valid passports.
pub fn count(input: &str) -> anyhow::Result<i32> {
    todo!()
}

use anyhow::Result;
fn main() -> Result<()> {
    let input = include_str!("../input");
    let count = count(input);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn it_works() {
        let valid = 1 + 1;
        assert_eq!(valid, 2);
    }
}
