use std::ops::{Add, Mul};

use crate::{
    puzzle::{Puzzle, PuzzleResult},
    utils::grid::transpose,
};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let lines: Vec<&str> = input.lines().collect();
        let (operators_line, numbers_lines) = lines.split_last().expect("Expected multiple lines");

        let operators: Vec<u8> = operators_line
            .split_whitespace()
            .map(|o| o.as_bytes()[0])
            .collect();
        let rows = numbers_lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|element| {
                        element
                            .parse()
                            .expect(&format!("Value is not a number: {element}"))
                    })
                    .collect()
            })
            .collect();

        let res = calculate(transpose(rows), operators);
        return Ok(res.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let lines: Vec<&str> = input.lines().collect();

        let mut numbers = Vec::new();
        let mut number_arrays = Vec::new();
        let mut operators = Vec::new();

        for i in (0..lines[0].len()).rev() {
            let col: Vec<u8> = lines.iter().map(|l| l.as_bytes()[i]).collect();

            if col.iter().all(|c| c == &b' ') {
                continue;
            }

            let (&operator, digits) = col.split_last().expect("Expected multiple lines");
            let number: u64 = str::from_utf8(digits)?.trim().parse()?;
            numbers.push(number);

            if operator != b' ' {
                operators.push(operator);
                number_arrays.push(numbers.clone());
                numbers.clear();
            }
        }

        let res = calculate(number_arrays, operators);
        return Ok(res.to_string());
    }
}

fn calculate(number_arrays: Vec<Vec<u64>>, operators: Vec<u8>) -> u64 {
    number_arrays
        .iter()
        .zip(operators)
        .map(|(numbers, operator)| {
            let op: fn(u64, u64) -> u64 = match operator {
                b'+' => u64::add,
                b'*' => u64::mul,
                _ => panic!("Unexpected operator: {operator}"),
            };
            let res = numbers.iter().copied().reduce(op).expect("Column is empty");
            res
        })
        .sum()
}
