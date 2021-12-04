use std::{fs, time::{Duration, Instant}, fmt::Debug};

pub fn read_file_into_line_array(filename: &str) -> Vec<String>
{
    read_string_into_line_array(fs::read_to_string(filename).expect("Can't read file"))
}

pub fn read_string_into_line_array(string: String) -> Vec<String>
{
    string.split('\n').map(str::to_string).collect()
}

pub fn read_str_into_line_array(string: &str) -> Vec<String>
{
    string.split('\n').map(str::to_string).collect()
}

pub fn timed_execute<T, F>(function: F ) -> (Duration, T)
where F: Fn() -> T
{
    let start = Instant::now();
    let result = function();
    let duration = start.elapsed();
    (duration, result)
}

pub fn print_result_timed_execute<T, F>(function: F, name: &str) -> T
where
    F: Fn() -> T,
    T: Debug
{
    let (duration, result) = timed_execute(function);
    println!("{}: result {:?} in {:?}us", name, result, duration.as_micros());
    result
}

pub fn print_timed_execute<T, F>(function: F, name: &str) -> T
where
    F: Fn() -> T,
    T: Debug
{
    let (duration, result) = timed_execute(function);
    println!("{}: in {:?}us", name, duration.as_micros());
    result
}
