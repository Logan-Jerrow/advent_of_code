mod length;
pub mod year;

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{alpha1, digit1, hex_digit1},
        combinator::{map_parser, map_res, opt},
        sequence::{pair, preceded},
        IResult,
    };

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
}

mod color {
    use nom::{error::Error, Finish};
    use std::borrow::Cow;

    use super::parser::color;

    pub struct Color<'a>(Cow<'a, str>);

    impl std::str::FromStr for Color<'_> {
        type Err = Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match color(s).finish() {
                Ok((_remaining, color)) => Ok(Color(color.to_string().into())),
                Err(Error { input, code }) => Err(Error {
                    input: input.to_string(),
                    code,
                }),
            }
        }
    }
}
mod id {
    use std::borrow::Cow;

    pub struct Id<'a>(Cow<'a, str>);
}

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
