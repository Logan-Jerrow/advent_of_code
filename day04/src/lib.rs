#![allow(dead_code)]
pub static DAY03_INPUT: &str = include_str!("../input");

pub mod passport {
    pub struct Passport {
        birth_year: String,
        issue_year: String,
        expiration_year: String,
        height: i32,
        hair_color: String,
        eye_color: String,
        passport_id: i64,
        country_id: Option<i64>,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
