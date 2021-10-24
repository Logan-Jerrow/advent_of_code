use nom::{error::Error, Finish};
use std::borrow::Cow;

use super::parser::id;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id<'a>(Cow<'a, str>);

impl Id<'_> {
    pub fn new(s: &str) -> Option<Id> {
        Some(Id(s.to_string().into()))
    }
}

impl std::str::FromStr for Id<'_> {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match id(s).finish() {
            Ok((_remaining, id)) => Ok(Id(id.to_string().into())),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_id() {
        assert_eq!("pid:756972774".parse::<Id>(), Ok(Id("756972774".into())));
    }
}
