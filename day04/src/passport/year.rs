use nom::{
    error::Error,
    Finish,
};

use super::parser::year;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Year(u16);

impl std::str::FromStr for Year {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match year(s).finish() {
            Ok((_remaining, year)) => Ok(Year(year)),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Year;
    #[test]
    fn birth_year() {
        const INPUT: &str = "byr:2021";
        let birth = INPUT.parse::<Year>();
        assert!(birth.is_ok());
        assert_eq!(birth.unwrap().0, 2021);
    }
}
