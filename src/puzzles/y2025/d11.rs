use std::collections::HashMap;

use crate::{
    puzzle::{Puzzle, PuzzleResult},
    utils::split_and_parse,
};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let racks = parse_input(input);

        let res = count_paths(&racks, *b"you").total;

        return Ok(res.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let racks = parse_input(input);

        let res = count_paths(&racks, *b"svr");

        return Ok(res.both.to_string());
    }
}

type RackId = [u8; 3];

#[derive(Clone, Copy, Debug)]
struct Counts {
    total: u64,
    dac: u64,
    fft: u64,
    both: u64,
}

impl std::ops::Add for Counts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            total: self.total + rhs.total,
            dac: self.dac + rhs.dac,
            fft: self.fft + rhs.fft,
            both: self.both + rhs.both,
        }
    }
}

fn count_paths(racks: &HashMap<RackId, Vec<RackId>>, from: RackId) -> Counts {
    let mut out_counts = HashMap::<RackId, Counts>::new();
    out_counts.insert(
        *b"out",
        Counts {
            total: 1,
            dac: 0,
            fft: 0,
            both: 0,
        },
    );

    get_paths_count(&mut out_counts, &racks, from)
}

fn get_paths_count(
    out_counts: &mut HashMap<RackId, Counts>,
    racks: &HashMap<RackId, Vec<RackId>>,
    id: RackId,
) -> Counts {
    match out_counts.get(&id) {
        Some(count) => {
            return *count;
        }
        None => {
            let mut count = racks
                .get(&id)
                .expect(&format!("Id does not exist {:?}", str::from_utf8(&id)))
                .iter()
                .map(|out| get_paths_count(out_counts, racks, *out))
                .reduce(|a, b| a + b)
                .expect("Empty connections");
            if id == *b"dac" {
                count.dac = count.total;
                count.both = count.fft;
            }
            if id == *b"fft" {
                count.fft = count.total;
                count.both = count.dac;
            }
            out_counts.insert(id, count);
            return count;
        }
    };
}

fn parse_input(input: &str) -> HashMap<RackId, Vec<RackId>> {
    input
        .lines()
        .map(|line| {
            let [name_str, outputs_str]: [String; 2] =
                split_and_parse(line, ':').expect("Wrong input format");
            let id = name_str.trim().as_bytes().try_into().expect("Malformed id");
            let outputs = outputs_str
                .trim()
                .split_whitespace()
                .map(|o_name| o_name.as_bytes().try_into().expect("Malformed output id"))
                .collect();
            (id, outputs)
        })
        .collect()
}
