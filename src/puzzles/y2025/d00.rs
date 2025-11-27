use crate::puzzle::{Puzzle, PuzzleResult};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, _input: &str) -> PuzzleResult {
        return Ok("no solution".to_string());
    }

    fn solve2(&self, _input: &str) -> PuzzleResult {
        return Ok("no solution".to_string());
    }
}
