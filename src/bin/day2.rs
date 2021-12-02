use std::str::FromStr;

use advent_of_code_2021::read_file_into_line_array;

fn main() {

    enum Command
    {
        Forward(u32),
        Down(u32),
        Up(u32)
    }

    impl FromStr for Command
    {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split_whitespace();
            let command = parts.next().expect("No command on line");
            let value = parts.next().expect("No value on line").parse::<u32>().expect("Can't parse value");
            match command
            {
                "forward" => Ok(Command::Forward(value)),
                "down" => Ok(Command::Down(value)),
                "up" => Ok(Command::Up(value)),
                _ => Err(())
            }
        }
    }

    // Task 1
    let commands = read_file_into_line_array("input/day2.txt").into_iter().map(|s| Command::from_str(s.as_str()).expect("Invalid command found"));

    let (depth, distance) = commands.fold((0u32, 0u32), |(depth, distance), command|
    {
        match command
        {
            Command::Forward(val) => (depth, distance + val),
            Command::Down(val) => (depth + val, distance),
            Command::Up(val) => (depth - val, distance),
        }
    });
    println!("Task 1: Depth {:?}, distance {:?}, multiplied {:?}",depth, distance, depth*&distance);

    // Task 2
    let commands = read_file_into_line_array("input/day2.txt").into_iter().map(|s| Command::from_str(s.as_str()).expect("Invalid command found"));

    let (depth, distance, _) = commands.fold((0u32, 0u32, 0u32), |(depth, distance, aim), command|
    {
        match command
        {
            Command::Forward(val) => (depth + val*aim, distance + val, aim),
            Command::Down(val) => (depth, distance, aim+val),
            Command::Up(val) => (depth, distance, aim-val),
        }
    });
    println!("Task 2: Depth {:?}, distance {:?}, multiplied {:?}",depth, distance, depth*&distance);

}
