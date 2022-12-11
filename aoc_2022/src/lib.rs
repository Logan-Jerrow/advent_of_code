/*
   --- Day 1: Calorie Counting ---

Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to
deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that
only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove
where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by
December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab
any fruit you see along the way, just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star.
Good luck!

---------

The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the
Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking
inventory of their supplies. One important consideration is food - in particular, the number of
Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks,
rations, etc. that they've brought with them, one item per line. Each Elf separates their own
inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following
list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

This list represents the Calories of the food carried by five Elves:

The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories. The
second Elf is carrying one food item with 4000 Calories. The third Elf is carrying food with 5000
and 6000 Calories, a total of 11000 Calories. The fourth Elf is carrying food with 7000, 8000, and
9000 Calories, a total of 24000 Calories. The fifth Elf is carrying one food item with 10000
Calories.

    In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd
    like to know how many Calories are being carried by the Elf carrying the most Calories. In the
    example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

--- Part Two ---

By the time you calculate the answer to the Elves' question, they've already realized that the Elf
carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories
carried by the top three Elves carrying the most Calories. That way, even if one of those Elves
runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third
Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories
carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in
total?

*/

pub mod day01 {
    use std::str::FromStr;

    const day01: &str = include_str!("../input/day01.txt");

    pub fn part1() -> u32 {
        most_calories(&process_elves(day01)).unwrap().total()
    }

    pub fn part2() -> u32 {
        most_3_total(&process_elves(day01))
    }

    fn process_elves(input: &str) -> Vec<Elf> {
        // let lines = input.lines().filter(|l| !l.is_empty());
        let lines = input.split("\n\n");

        let mut elfs: Vec<Elf> = Vec::with_capacity(lines.clone().count());
        for line in lines {
            let elf = line.parse::<Elf>().unwrap();
            elfs.push(elf);
        }
        elfs
    }

    fn most_calories(elves: &[Elf]) -> Option<&Elf> {
        elves.iter().max()
    }

    fn most_3_total(elves: &[Elf]) -> u32 {
        let mut cal: Vec<u32> = elves.iter().map(|e| e.total()).collect();
        cal.sort();

        let count = cal.len();
        cal[count - 3..].iter().sum()
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Elf(Vec<u32>);

    impl PartialOrd for Elf {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let s: u32 = self.0.iter().sum();
            let o: u32 = other.0.iter().sum();

            s.partial_cmp(&o)
        }
    }

    impl Elf {
        pub fn total(&self) -> u32 {
            self.0.iter().sum::<u32>()
        }
    }

    impl Ord for Elf {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let s: u32 = self.0.iter().sum();
            let o: u32 = other.0.iter().sum();

            s.cmp(&o)
        }
    }

    impl FromStr for Elf {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let fruit_count = s.matches('\n').count();
            let mut calories: Vec<u32> = Vec::with_capacity(fruit_count);

            for c in s.lines() {
                calories.push(c.parse().unwrap());
            }
            Ok(Elf(calories))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        #[test]
        fn test() {
            let elfs: Vec<Elf> = process_elves(INPUT);

            let most = most_calories(&elfs).unwrap();
            assert_eq!(most.total(), 24000);
        }
    }
}
