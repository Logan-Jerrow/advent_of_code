use nom::{error::Error, Finish};
use std::borrow::Cow;

use super::parser::color_redo;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color<'a>(pub Cow<'a, str>);

impl Color<'_> {
    pub fn new(s: &str) -> Option<Color> {
        Some(Color(s.to_string().into()))
    }
}

impl std::str::FromStr for Color<'_> {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match color_redo(s).finish() {
            Ok((_remaining, color)) => Ok(Color(color.to_string().into())),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    #[test]
    fn eye_color() {
        assert_eq!("ecl:gry".parse::<Color>(), Ok(Color("gry".into())));
    }
}
