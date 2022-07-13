pub static DAY03_INPUT: &str = include_str!("../input");

pub mod point {

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    impl Point {
        pub fn new(x: usize, y: usize) -> Self {
            Point { x, y }
        }
    }

    impl std::ops::AddAssign for Point {
        fn add_assign(&mut self, other: Self) {
            *self = Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
}

use point::Point;

const START: Point = Point {
    x: 0_usize,
    y: 0_usize,
};

pub fn traverse(slope: &Point, map: &str) -> i64 {
    let mut map = map.lines().step_by(slope.y).skip(1).peekable();
    let length = map.peek().unwrap().len();
    let mut position = START;
    let mut count: i64 = 0;
    for line in map {
        position += *slope;
        let mut index = position.x;

        if position.x >= length {
            index = position.x % length;
        }

        if let Some('#') = line.chars().nth(index) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod day03 {
    use super::*;

    pub const MAP: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn part1() {
        let count = traverse(&Point::new(3, 1), MAP);
        assert_eq!(count, 7);
    }
    #[test]
    fn part2() {
        let count = traverse(&Point::new(3, 1), MAP);
        let answer1 = traverse(&Point::new(1, 1), MAP);
        let answer2 = traverse(&Point::new(5, 1), MAP);
        let answer3 = traverse(&Point::new(7, 1), MAP);
        let answer4 = traverse(&Point::new(1, 2), MAP);

        let result = answer1 * count * answer2 * answer3 * answer4;

        assert_eq!(result, 336);
    }
}
