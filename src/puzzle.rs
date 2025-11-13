use std::time::Instant;
use core::time::Duration;

pub trait Puzzle{
    fn solution1(&self, input: &str) -> String;
    fn solution2(&self, input: &str) -> String;
    
    fn solve1(&self, input: &str) -> (String, Duration) {
        let start = Instant::now();
        let result = self.solution1(input);
        (result, start.elapsed())
    }
    
    fn solve2(&self, input: &str) -> (String, Duration) {
        let start = Instant::now();
        let result = self.solution2(input);
        (result, start.elapsed())
    }
}