use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, usize};

pub mod grid;

pub fn read_input(year: u16, day: u16, is_test: bool) -> Result<String, std::io::Error> {
    let filename = if is_test {
        format!("d{:02}_test.txt", day)
    } else {
        format!("d{:02}.txt", day)
    };

    let path: PathBuf = ["inputs", year.to_string().as_str(), &filename]
        .iter()
        .collect();

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

pub fn read_grid_parsed<T>(input: &str, start_at: usize) -> Result<Vec<Vec<T>>, T::Err>
where
    T: FromStr,
    T::Err: Display,
{
    input
        .lines()
        .enumerate()
        .filter(|&(i, _)| i >= start_at)
        .map(|(i, line)| {
            line.split_whitespace()
                .map(|d| {
                    d.parse()
                        .inspect_err(|e| eprintln!("Failed to parse <{d}> on line {i}: {e}"))
                })
                .collect()
        })
        .collect()
}

pub fn count_elements<T>(collection: impl IntoIterator<Item = T>) -> HashMap<T, u64>
where
    T: Eq + std::hash::Hash,
{
    let mut m = HashMap::new();
    for e in collection.into_iter() {
        let entry = m.entry(e).or_default();
        *entry += 1;
    }

    return m;
}

pub fn split_and_parse<T: FromStr + Debug, const N: usize>(
    input: &str,
    pattern: char,
) -> Result<[T; N], T::Err> {
    let split_result: Result<Vec<T>, _> = input.split(pattern).map(|i| i.parse::<T>()).collect();
    if split_result.is_err() {
        eprintln!("Cannot split and parse input '{input}' with delimiter '{pattern}'")
    }

    split_result.map(|split| {
        split.try_into().expect(&format!(
            "`split_and_parse` input does not match expected size ({N}): {input}"
        ))
    })
}

pub fn strip_brackets(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
