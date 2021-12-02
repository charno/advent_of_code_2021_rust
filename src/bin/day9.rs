use advent_of_code_2021::read_file_into_line_array;

fn main() {
    let values: Vec<u64> = read_file_into_line_array("input/day9.txt")
        .into_iter()
        .map(|s| s.parse::<u64>().expect("Invalid command found"))
        .collect();

    let mut end_index = 25usize;

    'outer: while end_index < values.len() {
        let start_index = end_index - 25;
        let mut slice = &values[start_index..end_index];

        while slice.len() > 1 {
            for current_index in 0..slice.len() - 1 {
                if slice[current_index] + slice[slice.len() - 1] == values[end_index] {
                    end_index = end_index + 1;
                    continue 'outer;
                }
            }
            slice = &slice[..&slice.len() - 1]
        }

        println!("Task 1: {:?}", values[end_index]);
        break;
    }

    let wrong_number = values[end_index];

    'outer2: for window_size in 2..values.len()
    {
        for start_position in 0..values.len()-window_size
        {
            let slice = &values[start_position..start_position+window_size];
            let sum: u64 = slice.iter().sum();
            if sum == wrong_number
            {
                let smallest = slice.iter().min().unwrap();
                let biggest = slice.iter().max().unwrap();
                println!("Task 2: Smallest={:?}, Biggest={:?}, Sum={:?}", smallest, biggest, smallest+biggest);
                break 'outer2;
            }
        }
    }


}
