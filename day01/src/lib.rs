pub fn find_entries_sum_multi(entries: &[i32], sum: i32) -> Option<i32> {
    for &e in entries {
        for &b in entries {
            if e + b == sum {
                return Some(e * b);
            }
        }
    }
    None
}

#[test]
fn day01() {
    let _x = "Hello world ";
    let report = vec![1721, 979, 366, 299, 675, 1456];
    let answer = find_entries_sum_multi(&report, 2020);
    assert_eq!(answer, Some(514579));
}
