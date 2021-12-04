use std::str::FromStr;

use advent_of_code_2021::read_file_into_line_array;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct PlayField
{
    playfield: [(u8, bool); 25]
}

impl PlayField {
    fn set_value_and_check_win(&mut self, value: u8) -> bool
    {
        // Set the value
        for (val, set) in self.playfield.iter_mut()
        {
            if val == &value
            {
                *set = true;
            }
        };

        self.check_win_condition()
    }

    fn check_win_condition(&self) -> bool {
        // Check win condition per line
        for line in self.playfield.chunks(5)
        {
            match line.iter().find(|&&(_, set)| set == false)
            {
                Some(_) => (),
                None => return true,
            }
        }

        // Check win condition per row
        for row in 0..5
        {
            let mut iter = self.playfield.iter();
            if row > 0
            {
                iter.nth(row-1);
            }

            match iter.step_by(5).find(|&&(_, set)| set == false)
            {
                Some(_) => (),
                None => return true,
            }
        }
        false
    }

    fn get_sum_of_unchecked(&self) -> u64 {
        self.playfield.iter().fold(0u64, |acc, &(value, set)| acc + if !set {value as u64} else {0})
    }
}

impl FromStr for PlayField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = s.split_ascii_whitespace();

        let mut result = PlayField {playfield: [(0, false); 25]};


        for i in 0..25
        {
            match cells.next()
            {
                Some(val) => result.playfield[i] = (u8::from_str(val).unwrap(), false),
                None => return Err(()),
                }
        }
        Ok(result)
    }
}


fn task2(input: &(Vec<PlayField>, Vec<u8>)) -> u64
{
    let (playfields, inputs) = input;
    let mut playfields = (*playfields).clone();

    for &value in inputs
    {
        // The last present playfield
        if playfields.len() == 1
        {
            if playfields[0].set_value_and_check_win(value)
            {
                return (value as u64) * playfields[0].get_sum_of_unchecked();
            }
        }
        // play all playfields, and sort out the ones that are already won
        else
        {
            for playfield in playfields.iter_mut()
            {
                playfield.set_value_and_check_win(value);
            }
            playfields.retain(|playfield| !playfield.check_win_condition());
        }

    }
    0
}

fn task1(input: &(Vec<PlayField>, Vec<u8>)) -> u64
{
    let (playfields, inputs) = input;
    let mut playfields = (*playfields).clone();

    for &value in inputs
    {
        for playfield in playfields.iter_mut()
        {
            if playfield.set_value_and_check_win(value)
            {
                return (value as u64) * playfield.get_sum_of_unchecked();
            }
        }
    }
    0
}

fn prepare_data(input: &Vec<String>) -> (Vec<PlayField>, Vec<u8>)
{
    let firstline = input[0].split(",").map(|s| u8::from_str(s).unwrap()).collect_vec();

    let play_fields_lines = &input[2..];
    let play_fields = play_fields_lines.chunks(6).map(|chunk| chunk.join(" ")).map(|s| PlayField::from_str(s.as_str()).unwrap()).collect_vec();

    (play_fields, firstline)
}

fn main() {
    let inputdata = prepare_data(&read_file_into_line_array("input/day4.txt"));
    println!("Day 4 - Task 1: {:?}, Task 2: {:?}", task1(&inputdata), task2(&inputdata));
}

#[cfg(test)]
mod tests
{
    use advent_of_code_2021::read_file_into_line_array;
    use itertools::Itertools;


    #[test]
    fn task1_testdata()
    {
        assert_eq!(super::task1(&super::prepare_data(&get_test_data())), 4512)
    }

    #[test]
    fn task2_testdata()
    {
        assert_eq!(super::task2(&super::prepare_data(&get_test_data())), 1924)
    }

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&super::prepare_data(&read_file_into_line_array("input/day4.txt"))), 65325)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&super::prepare_data(&read_file_into_line_array("input/day4.txt"))), 4624)
    }

    fn get_test_data() -> Vec<String>
    {
        TESTDATA.split("\n").map(|s| s.to_string()).collect_vec()
    }

    static TESTDATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
}