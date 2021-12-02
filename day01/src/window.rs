use crate::depth::*;
struct Window {
    first: Option<u32>,
    second: Option<u32>,
    third: Option<u32>,
}

impl Window {
    fn total(&self) -> Option<u32> {
        Some(self.first? + self.second? + self.third?)
    }

    fn depth(&self, other: &Self) -> Option<Depth> {
        match self.total()?.cmp(&other.total()?) {
            std::cmp::Ordering::Less => Some(Depth::Increased),
            std::cmp::Ordering::Equal => Some(Depth::NoChange),
            std::cmp::Ordering::Greater => Some(Depth::Decreased),
        }
    }
}
