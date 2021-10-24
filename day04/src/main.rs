#![allow(unused_variables, dead_code, rust_2018_idioms)]

pub mod passport;
/// Count the number of valid passports.
pub fn count(input: &str) -> anyhow::Result<usize> {
    let raws = input.split("\n\n").collect_vec();

    let passports = raws
        .iter()
        .map(|p| passport::parse_passport(p).finish())
        .filter_ok(|(_, p)| p.is_some())
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

    let count_part2 = count_redo(input);
    println!("valid part 2 passports: {}", count_part2?);
    
    Ok(())
}

fn count_redo(input: &str) -> anyhow::Result<usize> {
    let raws = input.split("\n\n").collect_vec();

    let passports = raws
        .iter()
        .map(|p| passport::parse_passport_redo(p).finish())
        .filter_ok(|(_, p)| p.is_some())
        .filter(|p| p.is_ok())
        .count();
    Ok(passports)
        // .collect_vec();
    // println!();
    // dbg!(passports);
    // todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let valid = count_redo(input);
        dbg!(&valid);
        assert_eq!(valid.unwrap(), 4);
    }

    #[test]
    fn invalid() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let invalid = count_redo(input);
        dbg!(&invalid);
        assert_eq!(invalid.unwrap(), 0);
    }
}
