use super::parser::length;
use nom::{error::Error, Finish};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Length {
    Cm(u16),
    In(u16),
    Unknown(u16),
    Unspecified(u16),
}

impl Length {
    pub fn new(number: &str, unit: Option<&str>) -> Option<Length> {
        let number = number.parse::<u16>().ok()?;
        match unit {
            Some("cm") => Some(Length::Cm(number)),
            Some("in") => Some(Length::In(number)),
            Some(_) => Some(Length::Unknown(number)),
            None => Some(Length::Unspecified(number)),
        }
    }
}

impl std::str::FromStr for Length {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match length(s).finish() {
            Ok((_remaining, (length, unit))) => match unit {
                Some("cm") => Ok(Length::Cm(length)),
                Some("in") => Ok(Length::In(length)),
                Some(_) => Ok(Length::Unknown(length)),
                None => Ok(Length::Unspecified(length)),
            },
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Length;

    #[test]
    fn parse_length_cm() {
        const INPUT: &str = "hgt:180cm";
        let height = INPUT.parse::<Length>();
        assert!(height.is_ok());
        assert_eq!(height.unwrap(), Length::Cm(180));
    }

    #[test]
    fn parse_length_in() {
        let input = "hgt:185in ecl:gry";
        let height = input.parse::<Length>();
        assert_eq!(height, Ok(Length::In(185)));
    }

    #[test]
    fn parse_length_unspecified() {
        let input = "hgt:100";
        let height = input.parse::<Length>();
        assert_eq!(height, Ok(Length::Unspecified(100)));
    }

    #[test]
    fn parse_length_unknown() {
        let input = "hgt:6ft";
        let height = input.parse::<Length>();
        assert_eq!(height, Ok(Length::Unknown(6)));
    }
}
