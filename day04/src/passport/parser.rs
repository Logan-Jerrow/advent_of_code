use std::str::FromStr;

use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take, take_while_m_n},
    character::complete::{
        alpha0, alpha1, alphanumeric0, alphanumeric1, digit1, hex_digit1, multispace0, multispace1,
        one_of,
    },
    combinator::{eof, map_parser, map_res, opt, recognize},
    multi::many1,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use super::{color::Color, id::Id, length::Length, year::Year, Passport};

pub fn parse_passport_redo(input: &str) -> IResult<&str, Option<Passport>> {
    /*
    let (i, byr) = byr(input)?;
    let (i, iyr) = iyr(i)?;
    let (i, eyr) = eyr(i)?;
    let (i, hgt) = hgt(i)?;
    let (i, hcl) = hcl(i)?;
    let (i, ecl) = ecl(i)?;
    let (i, pid) = pid(i)?;
    */
    let out = permutation((
        delimited(multispace0, byr, multispace0),
        delimited(multispace0, iyr, multispace0),
        delimited(multispace0, eyr, multispace0),
        delimited(multispace0, hgt, multispace0),
        delimited(multispace0, hcl, multispace0),
        delimited(multispace0, ecl, multispace0),
        delimited(multispace0, pid, multispace0),
        delimited(
            multispace0,
            opt(pair(tag("cid:"), alphanumeric0)),
            multispace0,
        ),
    ))(input)?;
    // Ok((i, Passport::new(byr, iyr, eyr, hgt, hcl, ecl, pid, None)))
    Ok((
        out.0,
        Passport::new(
            out.1 .0, out.1 .1, out.1 .2, out.1 .3, out.1 .4, out.1 .5, out.1 .6, None,
        ),
    ))
}

// fn ws(input: &str) -> IResult<&str, char> {
//     let (i, j) = one_of(" \n")(input)?;
// }

fn pid(input: &str) -> IResult<&str, Option<Id>> {
    let (i, _) = tag("pid:")(input)?;
    let (i, raw) = digit1(i)?;

    if raw.len() == 9 {
        return Ok((i, Some(super::id::Id(raw.into()))));
    }
    Ok((i, None))
}

fn ecl(input: &str) -> IResult<&str, Option<Color>> {
    let (i, _) = tag("ecl:")(input)?;
    let (i, raw) = alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(i)?;
    Ok((i, Some(super::color::Color(raw.into()))))
}

fn hcl(input: &str) -> IResult<&str, Option<Color>> {
    let (i, _) = tag("hcl:#")(input)?;
    let (i, raw) = recognize(many1(one_of("0123456789abcdef")))(i)?;

    if raw.len() == 6 {
        return Ok((i, Some(super::color::Color(raw.into()))));
    }
    Ok((i, None))
}

fn hgt(input: &str) -> IResult<&str, Option<Length>> {
    let (i, _) = tag("hgt:")(input)?;
    let (i, raw) = digit1(i)?;
    let (i, unit) = alt((tag("cm"), tag("in")))(i)?;

    if unit == "cm" {
        if let Some(number) = number_limit(raw, 150, 193) {
            return Ok((i, Some(Length::Cm(number))));
        }
    }
    if unit == "in" {
        if let Some(number) = number_limit(raw, 59, 76) {
            return Ok((i, Some(Length::In(number))));
        }
    }
    Ok((i, None))
}

fn number_limit(input: &str, min: u16, max: u16) -> Option<u16> {
    if let Ok(num) = input.parse::<u16>() {
        if num >= min && num <= max {
            return Some(num);
        }
    }
    None
}

fn eyr(input: &str) -> IResult<&str, Option<Year>> {
    let (i, _) = tag("eyr:")(input)?;
    let (i, raw) = digit1(i)?;

    if raw.len() == 4 {
        if let Some(number) = number_limit(raw, 2020, 2030) {
            return Ok((i, Some(Year(number))));
        }
    }
    Ok((i, None))
}

fn iyr(input: &str) -> IResult<&str, Option<Year>> {
    let (i, _) = tag("iyr:")(input)?;
    let (i, raw) = digit1(i)?;

    if raw.len() == 4 {
        if let Some(number) = number_limit(raw, 2010, 2020) {
            return Ok((i, Some(Year(number))));
        }
    }
    Ok((i, None))
}

fn byr(input: &str) -> IResult<&str, Option<Year>> {
    let (i, _) = tag("byr:")(input)?;
    let (i, raw) = digit1(i)?;

    if raw.len() == 4 {
        if let Some(number) = number_limit(raw, 1920, 2002) {
            return Ok((i, Some(Year(number))));
        }
    }
    Ok((i, None))
}

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
        Passport::new(birth, issue, expire, height, hair, eye, pass, country),
    ))
}

#[cfg(test)]
mod test {
    use crate::passport::{color::Color, id::Id, length::Length, year::Year};

    use super::*;

    #[test]
    fn test_redo() {
        let input = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let something = parse_passport_redo(input);
        assert!(something.is_ok());
    }

    #[test]
    fn test_redo_1() {
        let input = "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        let something = parse_passport_redo(input);
        assert!(something.is_ok());
    }

    #[test]
    fn test_byr() {
        let y = byr("byr:2002").unwrap();
        assert_eq!(y.1, Some(Year(2002)));
        let y = byr("byr:2003").unwrap();
        assert!(y.1.is_none());
    }

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
