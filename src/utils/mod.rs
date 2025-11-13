use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub fn read_input(year: u16, day: u16, is_test: bool) -> Result<String, std::io::Error> {
    let filename = if is_test {
        format!("d{:02}_test.txt", day)
    } else {
        format!("d{:02}.txt", day)
    };
    
    let path: PathBuf = ["inputs", year.to_string().as_str(), &filename]
        .iter()
        .collect();
    println!("trying to read {:?}",path);
    
    fs::read_to_string(path)
}

pub fn read_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn read_numbers(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

pub fn format_duration(duration: Duration) -> String {
    let micros = duration.as_micros();
    let millis = duration.as_millis();
    let secs = duration.as_secs_f64();
    
    if micros < 1_000 {
        format!("{} µs", micros)
    } else if millis < 1_000 {
        format!("{:.2} ms", secs * 1000.0)
    } else {
        format!("{:.2} s", secs)
    }
}