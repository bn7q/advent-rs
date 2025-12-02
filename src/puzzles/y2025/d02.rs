use std::num::ParseIntError;

use crate::puzzle::{Puzzle, PuzzleResult};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let intervals = get_intervals(input)?;

        let res: u64 = intervals
            .into_iter()
            .map(|(from, to)| {
                let mut cnt = 0u64;
                for n in from..=to {
                    let s = n.to_string();
                    if s.len() % 2 == 0 {
                        let (l, r) = s.split_at(s.len() / 2);
                        if l == r {
                            cnt += n;
                        }
                    }
                }
                cnt
            })
            .sum();

        return Ok(res.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let intervals = get_intervals(input)?;

        let res: u64 = intervals
            .into_iter()
            .map(|(from, to)| {
                let mut cnt = 0u64;
                for n in from..=to {
                    let s = n.to_string();
                    for pattern_length in 1..=s.len() / 2 {
                        if s.len() % pattern_length == 0 {
                            let chunks =
                                s.as_bytes().chunks(pattern_length).collect::<Vec<&[u8]>>();
                            let Some((head, tail)) = chunks.split_first() else {
                                continue;
                            };
                            if tail.iter().all(|e| e == head) {
                                cnt += n;
                                break;
                            }
                        }
                    }
                }
                cnt
            })
            .sum();

        return Ok(res.to_string());
    }
}

fn get_intervals(input: &str) -> Result<Vec<(u64, u64)>, ParseIntError> {
    input
        .split(',')
        .map(|interval| {
            let parts: Result<Vec<u64>, ParseIntError> =
                interval.split('-').map(|i| i.parse()).collect();
            let [from, to] = parts?[0..2] else {
                panic!("Invalid input: {interval}")
            };
            Ok((from, to))
        })
        .collect()
}
