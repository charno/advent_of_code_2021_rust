use advent_of_code_2020::read_file_into_line_array;
use itertools::Itertools;

fn task1(earliest_departure: i32, buslines: &Vec<i32>) -> u64
{
    let (&busline, departuretime) = buslines.iter().map(
        |bus| (bus, ((earliest_departure/bus)+1)*bus)
    ).min_by_key(
        |tup| tup.1
    ).unwrap();

    (busline * (departuretime-earliest_departure)) as u64
}

fn task2(earliest_departure: i32, buslines: &Vec<i32>) -> u64
{
0
}


fn solve(input: &Vec<String>) -> (u64,u64)
{
    let mut iter = input.iter();
    let earliest_departure = iter.next().unwrap().parse().unwrap();
    let busdefinitions = iter.next().unwrap();
    let buslines = busdefinitions.split(',').filter(|x| x != &"x").map(|s| s.parse().unwrap()).collect_vec();

    (task1(earliest_departure, &buslines), task2(earliest_departure, &buslines))
}

fn main() {
    let (task1result, task2result) = solve(&read_file_into_line_array("input/day13.txt"));
    println!("Day 13 - Task 1: {:?}, Task 2: {:?}", task1result, task2result);
}

#[cfg(test)]
mod tests
{
    use itertools::Itertools;

    #[test]
    fn task1()
    {
        let (earliest_departure, buslines) = get_test_data();

        assert_eq!(super::task1(earliest_departure, &buslines), 295)
    }

    #[test]
    fn task2()
    {
        let (earliest_departure, buslines) = get_test_data();
        assert_eq!(super::task2(earliest_departure, &buslines), 286)
    }

    fn get_test_data() -> (i32, Vec<i32>) {
        let testdata =
        "939
        7,13,x,x,59,x,31,19";
        let mut iter = testdata.split_ascii_whitespace();
        let earliest_departure = iter.next().unwrap().parse().unwrap();
        let buslines = iter.next().unwrap().split(',').filter(|x| x != &"x").map(|s| s.parse().unwrap()).collect_vec();
        (earliest_departure, buslines)
    }
}