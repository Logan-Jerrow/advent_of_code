pub static DAY03_INPUT: &str = include_str!("../input");

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

struct Point {
    x: u32,
    y: u32,
}

const START: Point = Point { x: 0, y: 0 };

fn expand(map: &mut Vec<String>) {
    todo!()
}
pub fn traverse(x: u32, y: u32) {
    let slope = Point { x, y };
    let map = MAP.lines().map(String::from).collect::<Vec<String>>();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::MAP;
    #[test]
    fn it_works() {
        let m = MAP;
        assert_eq!(2 + 2, 4);
    }
}
