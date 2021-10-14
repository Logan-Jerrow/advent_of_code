#![allow(dead_code)]

struct PasswordPolicy {
    low: u8,
    high: u8,
    letter: char,
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
}

fn valid_count(list: &[&str]) -> i32 {
    let mut count = 0;
    for &parse in list {
        let mut iter = parse.split(':');
        let policy = PasswordPolicy::new(iter.next().expect("Can't split at ':'"));
        if policy.is_valid(iter.next().expect("Can't split password")) {
            count += 1;
        }
    }
    count
}

#[test]
fn day02() {
    let list = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let valid = valid_count(&list);
    assert_eq!(valid, 2);
}
