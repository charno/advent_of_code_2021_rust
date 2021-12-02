use std::fs;

pub fn read_file_into_line_array(filename: &str) -> Vec<String>
{
    fs::read_to_string(filename).expect("Can't read file").split('\n').map(str::to_string).collect()
}
