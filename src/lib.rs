use std::fs;

pub fn read_file_into_line_array(filename: &str) -> Vec<String>
{
    read_string_into_line_array(fs::read_to_string(filename).expect("Can't read file"))
}

pub fn read_string_into_line_array(string: String) -> Vec<String>
{
    string.split('\n').map(str::to_string).collect()
}
