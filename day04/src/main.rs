#![allow(unused_variables, dead_code, rust_2018_idioms)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod passport {
    use std::borrow::Cow;

    #[derive(Parser)]
    #[grammar = "passport.pest"]
    pub struct Passport<'a> {
        birth_year: Cow<'a, str>,
        issue_year: Cow<'a, str>,
        expiration_year: Cow<'a, str>,
        height: Cow<'a, str>,
        hair_color: Cow<'a, str>,
        eye_color: Cow<'a, str>,
        passport_id: Cow<'a, str>,
        country_id: Option<Cow<'a, str>>,
    }

    impl<'a> Passport<'a> {
        pub fn new<S>(raw: S) -> Passport<'a>
        where
            S: Into<Cow<'a, str>>,
        {
            todo!()
        }
    }
}

/// Count the number of valid passports.
pub fn count(input: &str) -> anyhow::Result<i32> {
    let passports: Vec<passport::Passport> = parse(input)?;
    todo!()
}

fn parse(s: &str) -> anyhow::Result<Vec<passport::Passport>> {
    let test = list_parser::eye_color("ecl:gry");

    let x = 3 * 5;
    todo!()
}

peg::parser!(grammar list_parser() for str {
    // ecl:gry
    pub rule eye_color() -> &'input str
        = "ecl:" three:$(['a'..='z']*<3>) {three}
});

use anyhow::Result;
fn main() -> Result<()> {
    let input = include_str!("../input");
    let count = count(&input);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_eye_color() {
        let color = list_parser::eye_color("ecl:gry");
        assert!(color.is_ok())
    }

    #[test]
    fn parse_eye_color_noise() {
        let color = list_parser::eye_color(
            "j890gpa3e a48ja8goa4e89j2ecl:gryj894a03tp8j9gjfdiap jfiaoptia3",
        );
        assert!(color.is_ok())
    }

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
        //let valid = count(&INPUT);
        //assert_eq!(valid, 2);
    }
}
