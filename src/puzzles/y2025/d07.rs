use crate::puzzle::{Puzzle, PuzzleResult};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let mut lines = input.lines().step_by(2);

        let init = lines.next().expect("Input shouldn't be empty");
        let len = init.len();
        let mut beams = init
            .as_bytes()
            .iter()
            .map(|b| b == &b'S')
            .collect::<Vec<bool>>()
            .clone();

        let mut splits = 0;

        for row in lines {
            let mut carryover = false;
            for i in 0..len {
                if beams[i] && row.as_bytes()[i] == b'^' {
                    beams[i - 1] = true;
                    beams[i] = false;
                    splits += 1;
                    carryover = true;
                } else {
                    if carryover {
                        beams[i] = true;
                        carryover = false;
                    }
                }
            }
        }

        return Ok(splits.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let mut lines = input.lines().step_by(2);

        let init = lines.next().expect("Input shouldn't be empty");
        let len = init.len();
        let mut beams = init
            .as_bytes()
            .iter()
            .map(|b| if b == &b'S' { 1 } else { 0 })
            .collect::<Vec<u64>>()
            .clone();

        for row in lines {
            let mut carryover = 0;
            for i in 0..len {
                if beams[i] > 0 && row.as_bytes()[i] == b'^' {
                    carryover = beams[i];
                    beams[i - 1] += carryover;
                    beams[i] = 0;
                } else {
                    if carryover > 0 {
                        beams[i] += carryover;
                        carryover = 0;
                    }
                }
            }
        }

        let res: u64 = beams.iter().sum();

        return Ok(res.to_string());
    }
}
