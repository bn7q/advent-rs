use std::{cmp::max, num::ParseIntError};

use crate::puzzle::{Puzzle, PuzzleResult};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let jotages: Result<Vec<u32>, ParseIntError> = input
            .lines()
            .map(|line| {
                let (hdc, ldc) = line
                    .chars()
                    .rev()
                    .fold((char::MIN, char::MIN), |(c1, c2), c| {
                        if c >= c1 || c2 == char::MIN {
                            (c, max(c1, c2))
                        } else {
                            (c1, c2)
                        }
                    });
                hdc.to_string() + ldc.to_string().as_str()
            })
            .map(|bat_str| bat_str.parse::<u32>())
            .collect();

        let res: u32 = jotages?.iter().sum();

        return Ok(res.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let jotages: Result<Vec<u64>, ParseIntError> = input
            .lines()
            .map(|line| {
                let output_chars = line.chars().rev().fold([char::MIN; 12], |mut chars, c| {
                    if chars[11] == char::MIN {
                        chars.rotate_right(1);
                        chars[0] = c;
                    } else {
                        let mut temp = c;
                        for i in 0..12 {
                            if temp >= chars[i] {
                                (chars[i], temp) = (temp, chars[i])
                            } else {
                                break;
                            }
                        }
                    }
                    chars
                });
                output_chars.iter().fold(String::new(), |output, char| {
                    output + char.to_string().as_str()
                })
            })
            .map(|bat_str| bat_str.parse::<u64>())
            .collect();

        let res: u64 = jotages?.iter().sum();

        return Ok(res.to_string());
    }
}
