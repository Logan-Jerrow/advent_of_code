pub mod color;
pub mod id;
pub mod length;
pub mod year;

mod parser;

pub use parser::parse_passport;

#[derive(Debug, PartialEq)]
pub struct Passport<'a> {
    birth_year: year::Year,
    issue_year: year::Year,
    expiration_year: year::Year,
    height: length::Length,
    hair_color: color::Color<'a>,
    eye_color: color::Color<'a>,
    passport_id: id::Id<'a>,
    country_id: Option<id::Id<'a>>,
}

impl<'a> Passport<'a>
{
    pub fn new(
        birth_year: Option<year::Year>,
        issue_year: Option<year::Year>,
        expiration_year: Option<year::Year>,
        height: Option<length::Length>,
        hair_color: Option<color::Color<'a>>,
        eye_color: Option<color::Color<'a>>,
        passport_id: Option<id::Id<'a>>,
        country: Option<id::Id<'a>>,
    ) -> Option<Passport<'a>>{
        Some(Passport{
            birth_year: birth_year?,
            issue_year: issue_year?,
            expiration_year: expiration_year?,
            height: height?,
            hair_color: hair_color?,
            eye_color: eye_color?,
            passport_id: passport_id?,
            country_id: country,
        })
    }
}

#[cfg(test)]
mod tests {
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
