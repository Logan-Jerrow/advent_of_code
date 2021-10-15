use day01::expense_report;

fn print_day(day: &str) {
    println!("# ==================================================");
    println!("# # {}", day);
    println!("# ==================================================");
}

pub fn day01() {
    print_day("Day 01");
    println!(
        "Find the two entries that sum to 2020; what do you get if you multiply them together?\n"
    );

    let input = day01::DAY01_INPUT.lines().collect::<Vec<&str>>();

    let answer = expense_report::find_product(&input, 2020).unwrap();
    println!("Part 1 puzzle answer is: {}", answer);

    let answer = expense_report::find_product3(&input, 2020).unwrap();
    println!("Part 2 puzzle answer is: {}", answer);

    println!();
}

pub fn day02() {
    print_day("Day 02");
    println!("How many passwords are valid according to their policies?\n");

    let input = day02::DAY02_INPUT.lines().collect::<Vec<&str>>();
    let answer = day02::valid_count(&input);
    println!("Part 1 answer: {}", answer);

    let answer = day02::valid_count_toboggan(&input);
    println!("Part 2 answer: {}", answer);

    println!();
}
