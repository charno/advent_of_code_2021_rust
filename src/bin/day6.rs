use advent_of_code_2021::{print_result_timed_execute, print_timed_execute};

type DataType = [u64;10];

fn task1(input: &DataType) -> u64
{
    count_all_the_fishes(input, 80)
}

fn count_all_the_fishes(input: &[u64; 10], days: u32) -> u64 {
    let mut fishes = input.clone();
    for _ in 1..=days
    {
        fishes[7] += fishes[0];
        fishes[9] += fishes[0];
        for i in 1..fishes.len()
        {
            fishes[i-1] = fishes[i]
        }
        fishes[9] = 0;
    }
    fishes.iter().sum()
}


fn task2(input: &DataType) -> u64
{
    count_all_the_fishes(input, 256)
}

static DATA: &str = include_str!("../../input/day6.txt");

fn prepare_data (data: &str) -> DataType
{
    let mut result : DataType = [0u64;10];
    for fish in data.split(",")
    {
        result[fish.parse::<usize>().unwrap()] += 1;
    }
    result
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
"3,4,3,1,2";

    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data(TESTDATA)), 5934);
    }
    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data(TESTDATA)), 26984457539);
    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(super::DATA)), 373378)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(super::DATA)), 1682576647495)
    }

}