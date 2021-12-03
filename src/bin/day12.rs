use std::str::FromStr;

use advent_of_code_2020::read_file_into_line_array;
use itertools::Itertools;

#[derive(Debug)]
enum Command
{
    North(i32, String),
    South(i32, String),
    East(i32, String),
    West(i32, String),
    Turn(i32, String),
    Forward(i32, String)
}

impl Command
{
    // position_direction: ((x, y), direction). Direction: 1=north, 2=east, 3=south, 4=west
    fn execute(&self, position_direction : ((i32, i32), i32) ) -> ((i32, i32), i32)
    {
        let ((x, y), direction) = position_direction;
        let result = match self
        {
            Command::North(val, _) => ((x, y + val), direction),
            Command::South(val, _) => ((x, y - val), direction),
            Command::East(val, _) => ((x + val, y), direction),
            Command::West(val, _) => ((x - val, y), direction),
            Command::Turn(val, _) => ((x,y), (direction + val) % 4),
            Command::Forward(val, _) =>
                {
                    let desc = "Execute of forward from next line".to_string();

                    match direction {
                        0 => Command::North(*val, desc).execute(position_direction),
                        1 => Command::East(*val, desc).execute(position_direction),
                        2 => Command::South(*val, desc).execute(position_direction),
                        3 => Command::West(*val, desc).execute(position_direction),
                        _ => panic!("Invalid value")
                    }
                }
        };

        //println!("Input: {:?}, command: {:?}, output: {:?}", position_direction, self, result);
        result
    }

        // position_direction: ((x, y), direction). Direction: 1=north, 2=east, 3=south, 4=west
        fn execute2(&self, position_waypoint : ((i32, i32), (i32, i32)) ) -> ((i32, i32), (i32, i32))
        {
            let (position, waypoint) = position_waypoint;
            let (wp_x, wp_y) = waypoint;
            let (pos_x, pos_y) = position;
            let result = match self
            {
                Command::North(val, _) => (position, (wp_x, wp_y+val)),
                Command::South(val, _) => (position, (wp_x, wp_y-val)),
                Command::East(val, _) => (position, (wp_x+val, wp_y)),
                Command::West(val, _) => (position, (wp_x-val, wp_y)),
                Command::Turn(val, _) => {

                    let mut new_waypoint = waypoint;
                    for i in 0..*val
                    {
                        new_waypoint = (new_waypoint.1, -new_waypoint.0)
                    }
                    (position, new_waypoint) // TODOturn waypoint around the ship
                }
                Command::Forward(val, _) => ((pos_x + wp_x*val, pos_y + wp_y*val), waypoint),
            };
            println!("Input: {:?}, command: {:?}, output: {:?}", position_waypoint, self, result);
            result
        }
}

impl FromStr for Command
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.chars().next().unwrap();
        let value = String::from(s)[1..].parse().unwrap();

        match command
        {
            'F' => Ok(Command::Forward(value, String::from(s))),
            'L' => {
                let turn = match value
                {
                    90 => 3,
                    180 => 2,
                    270 => 1,
                    _ => 0
                };
                Ok(Command::Turn(turn, String::from(s)))
            },
            'R' => {
                let turn = match value
                {
                    90 => 1,
                    180 => 2,
                    270 => 3,
                    _ => panic!("Invalid value")
                };
                Ok(Command::Turn(turn, String::from(s)))
            },
            'N' => Ok(Command::North(value, String::from(s))),
            'S' => Ok(Command::South(value, String::from(s))),
            'E' => Ok(Command::East(value, String::from(s))),
            'W' => Ok(Command::West(value, String::from(s))),
            _ => Err(())
        }
    }
}

fn task1(input: &Vec<Command>) -> u64
{
    let ((x, y), _) = input.iter().fold(((0i32,0i32), 1i32), |accumulator, element| element.execute(accumulator));
    return (x.abs()+y.abs()) as u64;
}

fn task2(input: &Vec<Command>) -> u64
{
    let ((x, y), _) = input.iter().fold(((0i32,0i32), (10i32, 1i32)), |accumulator, element| element.execute2(accumulator));
    return (x.abs()+y.abs()) as u64;
}


fn solve(input: &Vec<String>) -> (u64,u64)
{
    let input_commands = input.iter().map(|string| Command::from_str(string).expect("Could not parse line")).collect_vec();

    (task1(&input_commands), task2(&input_commands))
}

fn main() {
    let (task1result, task2result) = solve(&read_file_into_line_array("input/day12.txt"));
    println!("Day 12 - Task 1: {:?}, Task 2: {:?}", task1result, task2result);
}

#[cfg(test)]
mod tests
{
    use super::Command;
    use std::str::FromStr;
    use itertools::Itertools;

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&get_test_data()), 25)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&get_test_data()), 286)
    }

    fn get_test_data() -> Vec<Command> {
        let testdata =
        "F10
        N3
        F7
        R90
        F11";

        testdata.split_ascii_whitespace().map(|string| Command::from_str(string).expect("Could not parse line")).collect_vec()
    }
}