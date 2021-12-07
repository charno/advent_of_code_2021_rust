use advent_of_code_2021::{print_result_timed_execute, print_timed_execute};
use itertools::Itertools;

type DataType = Vec<i64>;

fn task1(input: &DataType) -> u64
{
    return (0..2000).map(
        |point| (
            input.iter().map(|val| (val-point).abs()).sum::<i64>(), point)
        ).min_by_key(
            |(distance, _)| *distance
        ).unwrap().0 as u64;
}


fn task2(input: &DataType) -> u64
{
    return (0..2000).map(
        |point| (
            input.iter().map(
                |val| {
                    let dist = (val-point).abs();
                    (dist * (1+dist)) / 2
                }
            ).sum::<i64>(), point)
        ).min_by_key(
            |(distance, _)| *distance
        ).unwrap().0 as u64;
}

static DATA: &str = include_str!("../../input/day7.txt");

fn prepare_data (data: &str) -> DataType
{
    data.split(",").map(|s| s.parse().unwrap()).collect_vec()
}

// Normal setup below

fn main() {
    println!("Day 5");
    let input = print_timed_execute(|| prepare_data(DATA), "Data prep") ;
    print_result_timed_execute(||task1(&input), "Task1");
    print_result_timed_execute(||task2(&input), "Task2");
}

#[cfg(test)]
mod tests
{

    static TESTDATA: &str =
"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data(TESTDATA)), 37);
    }
    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data(TESTDATA)), 168);
    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(super::DATA)), 359648)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(super::DATA)), 100727924)
    }

}