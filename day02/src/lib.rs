pub static DAY02_INPUT: &str = include_str!("../input");

struct PasswordPolicy {
    low: u8,
    high: u8,
    letter: char,
}

struct Password<'a> {
    password: &'a str,
}

impl PasswordPolicy {
    fn new(text: &str) -> Self {
        let mut iter = text.split_whitespace();
        let mut low_high = iter
            .next()
            .expect("Failed to get low high numbers")
            .split('-');
        PasswordPolicy {
            low: low_high
                .next()
                .expect("Failed low")
                .parse::<u8>()
                .expect("Parsing failed."),
            high: low_high
                .next()
                .expect("Failed high")
                .parse::<u8>()
                .expect("Parsing failed."),
            letter: iter
                .next()
                .expect("Failed letter")
                .chars()
                .next()
                .expect("Failed char"),
        }
    }
    fn is_valid(&self, password: &str) -> bool {
        let count = password.matches(self.letter).count();
        (count >= self.low.into()) && (count <= self.high.into())
    }

    fn is_valid_toboggan(&self, password: &Password) -> bool {
        let first = password
            .password
            .chars()
            .nth(usize::from(self.low - 1))
            .unwrap_or_default()
            == self.letter;
        let second = password
            .password
            .chars()
            .nth(usize::from(self.high - 1))
            .unwrap_or_default()
            == self.letter;
        first ^ second
    }
}

fn parse_policy_password<'a>(list: &'a [&str]) -> Vec<(PasswordPolicy, Password<'a>)> {
    let mut parse: Vec<(PasswordPolicy, Password)> = Vec::new();
    for &line in list {
        let mut iter = line.split(':');

        let policy = PasswordPolicy::new(iter.next().expect("Can't split at ':'"));
        // remove first leading space.
        let password = &iter.next().expect("Can't split password")[1..];

        parse.push((policy, Password { password }));
    }
    parse
}

pub fn valid_count(list: &[&str]) -> i32 {
    let mut count = 0;
    for &parse in list {
        let mut iter = parse.split(':');
        let policy = PasswordPolicy::new(iter.next().expect("Can't split at ':'"));
        let password = iter.next().expect("Can't split password");
        if policy.is_valid(password) {
            count += 1;
        }
    }
    count
}

pub fn valid_count_toboggan(list: &[&str]) -> i32 {
    let input = parse_policy_password(list);

    let mut count = 0;
    for (policy, password) in input {
        if policy.is_valid_toboggan(&password) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn valid() {
        let list = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let valid = valid_count(&list);
        assert_eq!(valid, 2);
    }

    #[test]
    fn toboggan() {
        let list = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let valid = valid_count_toboggan(&list);
        assert_eq!(valid, 1);
    }
}
