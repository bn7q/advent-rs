use std::num::ParseIntError;

use crate::puzzle::{Puzzle, PuzzleResult};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let arr = parse_input(input)?;
        let mut p: i64 = 50;
        let mut res = 0;

        for i in arr {
            p = (p + i) % 100;
            if p == 0 {
                res += 1;
            }
        }

        return Ok(res.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let arr = parse_input(input)?;
        let mut p: i64 = 50;
        let mut res = 0;

        for i in arr {
            res += (i + p).div_euclid(100).abs();
            p = (i + p).rem_euclid(100);
        }

        return Ok(res.to_string());
    }
}

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input
        .lines()
        .map(|line|{
            let (direction, count) = line.split_at(1);
            let n = match direction {
                "L" => -count.parse::<i64>()?,
                "R" => count.parse()?,
                _ => panic!("Unexpected input")
            };
            Ok(n)
        })
        .collect()
}