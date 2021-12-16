use std::{collections::HashMap};

use advent_of_code_2021::{print_result_timed_execute, print_timed_execute};
use itertools::Itertools;

type DataType = (HashMap<String, u64> /*pairs */, HashMap<String, (String, String)> /* rules */, char /* last char in input */);

fn task1((pairs, rules, last_char): &DataType) -> u64
{
    extract_polimer(pairs, rules, last_char, 10)
}

fn extract_polimer(pairs: &HashMap<String, u64>, rules: &HashMap<String, (String, String)>, last_char: &char, rounds: u64) -> u64 {
    let mut pairs = pairs.clone();
    for _ in 0..rounds
    {
        let mut target_pairs = HashMap::new();

        for (string, count) in pairs
        {
            *target_pairs.entry(rules[&string].0.clone()).or_insert(0) += count;
            *target_pairs.entry(rules[&string].1.clone()).or_insert(0) += count;
        }
        pairs = target_pairs;
    }
    let mut char_count: HashMap<char, u64> = HashMap::new();
    for (string, count) in pairs
    {
        *char_count.entry(string.chars().next().unwrap()).or_insert(0) += count;
    }
    *char_count.entry(*last_char).or_insert(0) += 1;
    let min_count = char_count.iter().map(|(_, count)| count).min().unwrap();
    let max_count = char_count.iter().map(|(_, count)| count).max().unwrap();
    max_count - min_count
}


fn task2((pairs, rules, last_char): &DataType) -> u64
{
    extract_polimer(pairs, rules, last_char, 40)
}

static DATA: &str = include_str!("../../input/day14.txt");

fn prepare_data (data: &str) -> DataType
{
    let mut pairs = HashMap::new();
    let mut rules = HashMap::new();
    let mut iter = data.split("\n");
    let mut last_char = 'a';

    // read starting text
    iter.next().unwrap().chars().collect::<Vec<char>>().windows(2).map(|chars| {
        let string = format!("{}{}", chars[0], chars[1]);
        last_char = chars[1];
        *pairs.entry(string).or_insert(0) += &1;
    }).collect_vec();

    // Skip empty line
    iter.next();

    for line in iter
    {
        let chars = line.chars().collect_vec();
        let from_pair = format!("{}{}", chars[0], chars[1]);
        let to_1 = format!("{}{}", chars[0], chars[6]);
        let to_2 = format!("{}{}", chars[6], chars[1]);
        rules.insert(from_pair, (to_1, to_2));
    }


    (pairs, rules, last_char)
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
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data(TESTDATA)), 1588);

    }
    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data(TESTDATA)), 2188189693529);

    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(super::DATA)), 4773)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(super::DATA)), 116985)
    }

}