use std::{collections::HashMap, str::FromStr};

use advent_of_code_2020::{print_result_timed_execute, read_str_into_line_array, print_timed_execute};
use itertools::Itertools;

#[derive(Debug)]
enum Command
{
    Mask(Mask),
    Mem(i64, i64),
}

#[derive(Debug,Clone)]
struct Mask
{
    pub zeroes: i64,
    pub ones: i64,
    pub floating: i64
}

impl Command
{
    fn execute(&self, memory : &mut HashMap<i64, i64>, mask: &mut Mask)
    {
        match self
        {
            Command::Mask(_mask) =>
            {
                mask.zeroes = _mask.zeroes;
                mask.ones = _mask.ones;
                mask.floating = _mask.floating;
            },
            Command::Mem(position, values) => {
                *memory.entry(*position).or_insert(0) = !(!(values | &mask.ones) | &mask.zeroes)
            }
        }
    }

    fn execute2(&self, memory : &mut HashMap<i64, i64>, mask: &mut Mask)
    {
        match self
        {
            Command::Mask(_mask) =>
            {
                mask.zeroes = _mask.zeroes;
                mask.ones = _mask.ones;
                mask.floating = _mask.floating;
            },
            Command::Mem(address, values) => {
                let address = address | mask.ones;

                let num_of_permutations = 2u64.pow(mask.floating.count_ones());

                if num_of_permutations > 1
                {
                    // Get the addresses of all floating bits
                    let mut addr_of_floating : Vec<u8> = Vec::new();
                    for i in 0..36
                    {
                        if mask.floating & (1 << i) != 0
                        {
                            addr_of_floating.push(i);
                        }
                    }

                    assert_eq!(mask.floating.count_ones() as usize, addr_of_floating.len());

                    for i in 0..=num_of_permutations
                    {
                        let mut address = address;

                        for j in 0..addr_of_floating.len()
                        {
                            if i & (1 << j) != 0
                            {
                                address |= 1 << addr_of_floating[j];
                            }
                            else
                            {
                                address = !((!address) | (1 << addr_of_floating[j]));
                            }
                        }

                        // TODO permutation!
                        *memory.entry(address).or_insert(0) = *values
                    }
                }
                else
                {
                    *memory.entry(address).or_insert(0) = *values
                }
            }
        }
    }
}

impl FromStr for Command
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4]
        {
            "mask" =>
            {
                let orig_mask = &s[7..];
                let zeroes = i64::from_str_radix(orig_mask.replace("1", "X").replace("0", "1").replace("X", "0").as_str(), 2).unwrap();
                let ones = i64::from_str_radix(orig_mask.replace("X", "0").as_str(), 2).unwrap();
                let floating = i64::from_str_radix(orig_mask.replace("1", "0").replace("X", "1").as_str(), 2).unwrap();
                Ok(Command::Mask(Mask {zeroes, ones, floating}))
            },
            "mem[" =>
            {
                let addr : i64 = s[4..s.find("]").unwrap()].parse().unwrap();
                let value : i64 = s[s.find("=").unwrap()+2..].parse().unwrap();
                Ok(Command::Mem(addr, value))
            },
            _ => Err(s.to_string())
        }
    }
}

fn task1(commands: &Vec<Command>) -> i64
{
    let mut memory : HashMap<i64, i64> = HashMap::new();
    let mut mask = Mask {zeroes: 0, ones: 0, floating: 0};

    for command in commands
    {
        command.execute(&mut memory, &mut mask);
    }

    return memory.iter().map(|(_, val)| *val).sum();

}

fn task2(commands: &Vec<Command>) -> i64
{
    let mut memory : HashMap<i64, i64> = HashMap::new();
    let mut mask = Mask {zeroes: 0, ones: 0, floating: 0};

    for command in commands
    {
        command.execute2(&mut memory, &mut mask);
    }

    return memory.iter().map(|(_, val)| *val).sum();
}





static DATA: &str = include_str!("../../input/day14.txt");

fn prepare_data (data: &str) -> Vec<Command>
{
    read_str_into_line_array(data).iter().map(|s| Command::from_str(s).unwrap()).collect_vec()
}

// Normal setup below

fn main() {
    println!("Day 14");
    let commands = print_timed_execute(|| prepare_data(DATA), "Data prep") ;
    print_result_timed_execute(||task1(&commands), "Task1");
    print_result_timed_execute(||task2(&commands), "Task2");
}

#[cfg(test)]
mod tests
{
    static TESTDATA: &str =
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    static TESTDATA2: &str =
"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data(TESTDATA)), 165)
    }

    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data(TESTDATA2)), 208)
    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(super::DATA)), 10885823581193)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(super::DATA)), 3816594901962)
    }

}