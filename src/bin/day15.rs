use advent_of_code_2020::{print_result_timed_execute, print_timed_execute};
use itertools::Itertools;

fn task1(input: &Vec<u64>) -> u64
{
    game_after_turns(input, 2020)
}


fn game_after_turns(input: &Vec<u64>, turns: u64) -> u64 {
    let mut field = vec![0usize;turns as usize];
    let mut turn = 1;
    let mut lastnumber = 0;
    for &number in input
    {
        lastnumber = number as usize;
        field[number as usize] = turn;
        turn += 1;
    }
    while turn <= turns as usize
    {
        let last_lastnumber = lastnumber;
        if field[lastnumber] != 0
        {
            lastnumber = turn - field[lastnumber] - 1;
        }
        else {
            lastnumber = 0;
        }
        field[last_lastnumber] = turn-1;
        turn += 1;
    }
    lastnumber as u64
}

fn task2(input: &Vec<u64>) -> u64
{
    game_after_turns(input, 30000000)
}

static DATA: &str = "6,19,0,5,7,13,1";

fn prepare_data (data: &str) -> Vec<u64>
{
    data.split(",").map(|s|s.parse().unwrap()).collect_vec()
}

// Normal setup below

fn main() {
    println!("Day 15");
    let input = print_timed_execute(|| prepare_data(DATA), "Data prep") ;
    print_result_timed_execute(||task1(&input), "Task1");
    print_result_timed_execute(||task2(&input), "Task2");
}

#[cfg(test)]
mod tests
{
    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data("0,3,6")), 436);
        assert_eq!(super::task1(&super::prepare_data("1,3,2")), 1);
        assert_eq!(super::task1(&super::prepare_data("2,1,3")), 10);
        assert_eq!(super::task1(&super::prepare_data("1,2,3")), 27);
        assert_eq!(super::task1(&super::prepare_data("2,3,1")), 78);
        assert_eq!(super::task1(&super::prepare_data("3,2,1")), 438);
        assert_eq!(super::task1(&super::prepare_data("3,1,2")), 1836);
    }

    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data("0,3,6")), 175594);
        assert_eq!(super::task2(&super::prepare_data("1,3,2")), 2578);
        assert_eq!(super::task2(&super::prepare_data("2,1,3")), 3544142);
        assert_eq!(super::task2(&super::prepare_data("1,2,3")), 261214);
        assert_eq!(super::task2(&super::prepare_data("2,3,1")), 6895259);
        assert_eq!(super::task2(&super::prepare_data("3,2,1")), 18);
        assert_eq!(super::task2(&super::prepare_data("3,1,2")), 362);
    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(super::DATA)), 468)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(super::DATA)), 1801753)
    }

}