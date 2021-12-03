use advent_of_code_2021::read_file_into_line_array;
use itertools::Itertools;


fn get_most_and_least_used_bits(input: &Vec<u16>) -> (u16, u16) {
    // Count all bits that are set to true
    let counted_bits = input.iter().fold([0usize; 16], |mut acc, entry| {
        for i in 0..acc.len()
        {
            acc[i] = acc[i] + if entry & (1 << acc.len() - i - 1 ) != 0 {1} else {0}
        }
        acc
    });

    // How many bits must be set to true, so that 1 is the most used bit?
    let need_more_or_equal_bits_than =
    match input.len()
    {
        1 => 1,
        len =>
            match len%2
            {
                0 => len/2,
                _ => len/2+1
            }
    };

    // Get the bitmask with the most used bits per positon
    let most_used_bits_in_position= counted_bits.iter().fold(0u16, |acc, bit_count|
    {
        (acc << 1) + if bit_count >= &need_more_or_equal_bits_than {1} else {0}
    });

    // Get the bitmask of which bits were used at all
    let bitmask_of_used_bits = counted_bits.iter().fold(0u16, |acc, bit_count|
    {
        (acc << 1) + if bit_count > &0usize || acc > 0 {1} else {0}
    });

    // Get the bitmask with the least used bits, by xoring the used bits with the most used bits
    let least_used_bits_in_position = most_used_bits_in_position ^ bitmask_of_used_bits;

    (most_used_bits_in_position, least_used_bits_in_position)
}

fn task1(input: &Vec<u16>) -> u64
{
    let (gamma_rate, epsilon_rate) = get_most_and_least_used_bits(input);
    gamma_rate as u64 * epsilon_rate as u64
}

fn task2(input: &Vec<u16>) -> u64
{
    // oxygen_bits is the bitmask of the most used bits, co2_bits the one with the least used bits
    let (mut oxygen_bits, mut co2_bits) = get_most_and_least_used_bits(input);

    // Copy the input vector, so that we can modify it
    let mut oxygen = input.clone();
    let mut co2 = input.clone();

    // From left to right...
    for i in (0..=15u16).rev()
    {
        // Set bitmask
        let bitmask = 1u16 << i;

        if oxygen.len() > 1
        {
            // Find all elements where the currently worked bit is the same as the most used bit in the remaining table
            oxygen = oxygen.iter().filter(|element| {*element & &bitmask == oxygen_bits & &bitmask}).map(|v|*v).collect_vec()
        }

        if co2.len() > 1
        {
            // Find all elements where the currently worked bit is the same as the least used bit in the remaining table
            co2 = co2.iter().filter(|element| {*element & &bitmask == co2_bits & &bitmask}).map(|v|*v).collect_vec()
        }

        // Update most/least used bit from the new tables
        oxygen_bits = get_most_and_least_used_bits(&oxygen).0;
        co2_bits = get_most_and_least_used_bits(&co2).1;
    }


    oxygen[0] as u64 * co2[0] as u64
}


fn solve(input: &Vec<String>) -> (u64,u64)
{
    let input_integers : Vec<u16> = input.iter().map(|string| u16::from_str_radix(string, 2).expect("Could not parse line")).collect();

    (task1(&input_integers), task2(&input_integers))
}

fn main() {
    let (task1result, task2result) = solve(&read_file_into_line_array("input/day3.txt"));
    println!("Day 3 - Task 1: {:?}, Task 2: {:?}", task1result, task2result);
}

#[cfg(test)]
mod tests
{

    #[test]
    fn task1()
    {
        assert_eq!(super::task1(&get_test_data()), 198)
    }

    #[test]
    fn task2()
    {
        assert_eq!(super::task2(&get_test_data()), 230)
    }

    fn get_test_data() -> Vec<u16> {
        let testdata =
        "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let input_integers : Vec<u16> = testdata.split_ascii_whitespace().map(|string| u16::from_str_radix(string, 2).expect("Could not parse line")).collect();
        input_integers
    }
}