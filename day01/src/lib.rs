pub static DAY01_INPUT: &str = include_str!("../input");

pub mod expense_report {
    fn parse_input(input: &[&str]) -> Vec<i32> {
        input
            .iter()
            .map(|&int| int.parse::<i32>().expect("Parsing error."))
            .collect::<Vec<i32>>()
    }

    pub fn find_product(entries: &[&str], sum: i32) -> Option<i32> {
        let numbers = parse_input(entries);
        for &e in &numbers {
            for &b in &numbers {
                if e + b == sum {
                    return Some(e * b);
                }
            }
        }
        None
    }

    pub fn find_product3(entries: &[&str], sum: i32) -> Option<i32> {
        let numbers = parse_input(entries);
        for &a in &numbers {
            for &b in &numbers {
                for &c in &numbers {
                    if a + b + c == sum {
                        return Some(a * b * c);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests_day01 {
    use crate::expense_report::*;

    const REPORT: [&str; 6] = ["1721", "979", "366", "299", "675", "1456"];

    #[test]
    fn product() {
        let _x = "Hello world ";
        let answer = find_product(&REPORT, 2020);
        assert_eq!(answer, Some(514579));
    }

    #[test]
    fn product3() {
        let _x = "Hello world ";
        let answer = find_product3(&REPORT, 2020);
        assert_eq!(answer, Some(241861950));
    }
}
