use std::{cmp::max, num::ParseIntError, ops::RangeInclusive};

use crate::puzzle::{Puzzle, PuzzleResult};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let (ranges, ingredients) = parse_input(input)?;

        let res = ingredients
            .iter()
            .filter(|ingridient| ranges.iter().any(|range| range.contains(ingridient)))
            .count();

        return Ok(res.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let (mut ranges, _) = parse_input(input)?;
        ranges.sort_by_key(|r| *r.start());

        let compressed_ranges =
            ranges
                .iter()
                .fold(Vec::<RangeInclusive<u64>>::new(), |mut ranges, range| {
                    if ranges
                        .last()
                        .iter()
                        .all(|latest_range| latest_range.end() < range.start())
                    {
                        ranges.push(range.clone());
                    } else {
                        ranges.last_mut().map(|latest_range| {
                            *latest_range = RangeInclusive::new(
                                *latest_range.start(),
                                max(*latest_range.end(), *range.end()),
                            )
                        });
                    }

                    ranges
                });

        let res: usize = compressed_ranges.iter().map(|r| r.clone().count()).sum();

        return Ok(res.to_string());
    }
}

fn parse_input(
    input: &str,
) -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>), Box<dyn std::error::Error>> {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        if line.contains('-') {
            let (from, to) = parse_range(line)?;
            ranges.push(from..=to);
        } else if line.len() > 0 {
            ingredients.push(line.parse::<u64>()?);
        }
    }

    Ok((ranges, ingredients))
}

fn parse_range(input: &str) -> Result<(u64, u64), ParseIntError> {
    let parts: Result<Vec<u64>, ParseIntError> = input.split('-').map(|i| i.parse()).collect();
    let [from, to] = parts?[0..2] else {
        panic!("Invalid input: {input}")
    };
    Ok((from, to))
}
