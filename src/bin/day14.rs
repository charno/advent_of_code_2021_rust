use std::{collections::HashMap, str::FromStr};

use advent_of_code_2020::read_file_into_line_array;
use bitmaps::Bitmap;
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

                let num_of_permutations = 2^mask.floating.count_ones();

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


fn solve(input: &Vec<String>) -> (i64,i64)
{
    let commands = input.iter().map(|s| Command::from_str(s).unwrap()).collect_vec();
    (task1(&commands), task2(&commands))
}

fn main() {
    let (task1result, task2result) = solve(&read_file_into_line_array("input/day14.txt"));
    println!("Day 14 - Task 1: {:?}, Task 2: {:?}", task1result, task2result);
}

#[cfg(test)]
mod tests
{
    use advent_of_code_2020::read_string_into_line_array;
    use itertools::Itertools;
    use super::Command;
    use std::str::FromStr;

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&get_test_data()), 165)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&get_test_data2()), 208)

    }

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

    fn get_test_data() -> Vec<Command> {

        let lines = read_string_into_line_array(TESTDATA.to_string());
        lines.into_iter().map(|s| Command::from_str(s.as_str()).unwrap()).collect_vec()
    }

    fn get_test_data2() -> Vec<Command> {

        let lines = read_string_into_line_array(TESTDATA2.to_string());
        lines.into_iter().map(|s| Command::from_str(s.as_str()).unwrap()).collect_vec()
    }
}