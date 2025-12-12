use crate::{
    puzzle::{Puzzle, PuzzleResult},
    utils::strip_brackets,
};

use itertools::Itertools;

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let machines = parse_input(input);

        let counts: usize = machines
            .iter()
            .map(|machine| {
                let mut n = 1;
                'outer: loop {
                    for btn_compbination in machine.buttons.clone().into_iter().combinations(n) {
                        let mut result_indicators = vec![false; machine.n];
                        for ind in btn_compbination.iter().flatten() {
                            result_indicators[*ind] = !result_indicators[*ind];
                        }
                        if result_indicators == machine.indicators {
                            break 'outer;
                        }
                    }
                    n += 1;
                    if n > machine.n {
                        panic!("no solution found");
                    }
                }
                n
            })
            .sum();

        return Ok(counts.to_string());
    }

    fn solve2(&self, _input: &str) -> PuzzleResult {
        return Ok("solution in python".to_string());
    }
}

#[derive(Debug)]
struct Machine {
    pub n: usize,
    pub indicators: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub _joltages: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let indicators_part = parts.next().expect("There's always first part");
            let n = indicators_part.len() - 2;
            let indicators = indicators_part
                .as_bytes()
                .iter()
                .flat_map(|i| match i {
                    b'.' => Some(false),
                    b'#' => Some(true),
                    _ => None,
                })
                .collect();

            let _joltages = parts
                .next_back()
                .map(|j_str| {
                    let mut chars = j_str.chars();
                    chars.next();
                    chars.next_back();
                    chars
                        .as_str()
                        .split(',')
                        .map(|j| j.parse::<usize>().expect("Expecting valid input"))
                })
                .expect("Joltages last part")
                .collect();

            let mut buttons = Vec::new();
            for btn_str in parts {
                let btn = strip_brackets(btn_str)
                    .split(',')
                    .map(|btn_i| btn_i.parse::<usize>().expect("valid input"))
                    .collect();
                buttons.push(btn);
            }

            Machine {
                n,
                indicators,
                buttons,
                _joltages,
            }
        })
        .collect()
}
