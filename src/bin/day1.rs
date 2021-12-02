use advent_of_code_2021::read_file_into_line_array;

use itertools::Itertools;
fn main() {
    // Task 1
    let integers = read_file_into_line_array("input/day1.txt").into_iter().map(|s| s.parse::<i32>().expect("Can't parse integer"));
    let map_of_is_bigger_than_last = integers.tuple_windows().map(|(first, second)| second > first);
    let count_of_items = map_of_is_bigger_than_last.counts();
    println!("Task 1: Depth is bigger than before {:?} times", count_of_items[&true]);

    // Task 2
    let integers = read_file_into_line_array("input/day1.txt").into_iter().map(|s| s.parse::<i32>().expect("Can't parse integer"));
    let map_of_is_bigger_than_last = integers.tuple_windows().map(|(first, second, third)| first+second+third).tuple_windows().map(|(first, second)| second > first);
    let count_of_items = map_of_is_bigger_than_last.counts();
    println!("Task 2: Depth with sliding window is bigger than before {:?} times", count_of_items[&true]);


}
