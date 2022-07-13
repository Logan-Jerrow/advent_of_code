pub use self::Depth::*;

#[derive(Debug, PartialEq)]
pub enum Depth {
    NA,
    Decreased,
    Increased,
    NoChange,
}
