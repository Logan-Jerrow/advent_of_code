use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while_m_n},
    character::complete::{alpha0, alpha1, alphanumeric1, digit1, hex_digit1, multispace1},
    combinator::{eof, map_parser, map_res, opt},
    multi::many1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

use super::Passport;

pub fn year(input: &str) -> IResult<&str, u16> {
    let (i, _) = tag("byr:")(input)?;
    let (i, year) = map_res(digit1, |out: &str| out.parse::<u16>())(i)?;

    Ok((i, year))
}

pub fn length(input: &str) -> IResult<&str, (u16, Option<&str>)> {
    let len = map_res(digit1, |number: &str| number.parse::<u16>());
    let unit = alt((tag("cm"), tag("in"), alpha1));

    let (i, _) = tag("hgt:")(input)?;
    let (i, len_unit) = pair(len, opt(unit))(i)?;

    Ok((i, len_unit))
}

pub fn color(input: &str) -> IResult<&str, &str> {
    let cl = alt((tag("hcl:"), tag("ecl:")));
    let mut hex = preceded(
        pair(cl, opt(tag("#"))),
        alt((hex_digit1, map_parser(alpha1, take(3u8)))),
    );

    let (i, color) = hex(input)?;

    Ok((i, color))
}

fn opt_pound(input: &str) -> IResult<&str, Option<&str>> {
    opt(tag("#"))(input)
}

fn color_tag(input: &str) -> IResult<&str, &str> {
    // colors come from ethier "hcl:" or "ecl:" and end in a optinal "#"
    terminated(alt((tag("hcl:"), tag("ecl:"))), opt_pound)(input)
}

fn hex_or_three_alpha(input: &str) -> IResult<&str, &str> {
    // colors are a hexedicimal or...
    // colors are three alphbetic characters
    alt((
        hex_digit1,
        take_while_m_n(3, 3, |c: char| c.is_alphabetic()),
    ))(input)
}

pub fn color_redo(input: &str) -> IResult<&str, &str> {
    preceded(color_tag, hex_or_three_alpha)(input)
}

fn id_tag(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    pair(alt((tag("pid:"), tag("cid:"))), opt_pound)(input)
}

pub fn id(input: &str) -> IResult<&str, &str> {
    preceded(id_tag, alphanumeric1)(input)
}

#[derive(Debug, PartialEq)]
enum PassportTags {
    Eyr,
    Byr,
    Iyr,
    Hgt,
    Ecl,
    Hcl,
    Pid,
    Cid,
}

impl std::str::FromStr for PassportTags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "eyr" => Ok(PassportTags::Eyr),
            "byr" => Ok(PassportTags::Byr),
            "iyr" => Ok(PassportTags::Iyr),
            "hgt" => Ok(PassportTags::Hgt),
            "ecl" => Ok(PassportTags::Ecl),
            "hcl" => Ok(PassportTags::Hcl),
            "pid" => Ok(PassportTags::Pid),
            "cid" => Ok(PassportTags::Cid),
            _ => Err(()),
        }
    }
}

fn parse_to_tags(input: &str) -> IResult<&str, PassportTags> {
    map_res(
        alt((
            tag("eyr"),
            tag("byr"),
            tag("iyr"),
            tag("hgt"),
            tag("ecl"),
            tag("hcl"),
            tag("pid"),
            tag("cid"),
        )),
        PassportTags::from_str,
    )(input)
}

#[derive(Debug)]
struct Raw<'a> {
    tag: PassportTags,
    opt_pound: Option<&'a str>,
    value: &'a str,
}

fn parse_to_raw(input: &str) -> IResult<&str, Raw> {
    match separated_pair(parse_to_tags, tag(":"), pair(opt_pound, alphanumeric1))(input) {
        Err(e) => Err(e),
        Ok((remainder, (tag, (opt_pound, value)))) => Ok((
            remainder,
            Raw {
                tag,
                opt_pound,
                value,
            },
        )),
    }
}

pub fn parse_passport(input: &str) -> IResult<&str, Option<Passport>> {
    /*
    eyr:2020
    byr:1937    tag : value
    iyr:2017

    hgt:183cm   tag : (value, unit)

    ecl:gry     tag : "#" value
    hcl:#fffffd

    pid:860033327   tag : "#" value
    cid:147
    */
    // Raw = tag:"#"(value)
    // Passport = 8 raw values
    // each raw is seperated by space or new line
    // each passport is seperated by two consecutive new line and will be dealt with in parent function

    let (i, raws) = many1(terminated(parse_to_raw, alt((multispace1, eof))))(input)?;
    // TODO: find a better way. just get it working
    let mut expire = None;
    let mut birth = None;
    let mut issue = None;
    let mut height = None;
    let mut eye = None;
    let mut hair = None;
    let mut pass = None;
    let mut country = None;

    for raw in raws {
        match raw.tag {
            PassportTags::Eyr => {
                expire = super::year::Year::new(raw.value);
            }
            PassportTags::Byr => {
                birth = super::year::Year::new(raw.value);
            }
            PassportTags::Iyr => {
                issue = super::year::Year::new(raw.value);
            }
            PassportTags::Hgt => {
                let (_, (num, unit)) = pair(digit1, opt(alpha0))(raw.value)?;
                height = super::length::Length::new(num, unit);
            }
            PassportTags::Ecl => {
                eye = super::color::Color::new(raw.value);
            }
            PassportTags::Hcl => {
                hair = super::color::Color::new(raw.value);
            }
            PassportTags::Pid => {
                pass = super::id::Id::new(raw.value);
            }
            PassportTags::Cid => {
                country = super::id::Id::new(raw.value);
            }
        }
    }
    Ok((
        i,
        Passport::new (
            birth,
            issue,
            expire,
            height,
            hair,
            eye,
            pass,
            country,
        ),
    ))
}

#[cfg(test)]
mod test {
    use crate::passport::{color::Color, id::Id, length::Length, year::Year};

    use super::parse_passport;

    #[test]
    fn test_parse_passport() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm";

        let (remainding, passport) = parse_passport(input).unwrap();
        assert!(remainding.is_empty());
        assert_eq!(
            passport.unwrap(),
            super::super::Passport {
                birth_year: Year::new("1937").unwrap(),
                expiration_year: Year::new("2020").unwrap(),
                issue_year: Year::new("2017").unwrap(),
                country_id: Id::new("147"),
                eye_color: Color::new("gry").unwrap(),
                hair_color: Color::new("fffffd").unwrap(),
                height: Length::Cm(183),
                passport_id: Id::new("860033327").unwrap(),
            }
        )
    }
}
