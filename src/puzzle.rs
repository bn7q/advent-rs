use std::error::Error;

pub type PuzzleResult = Result<String, Box<dyn Error>>;

pub trait Puzzle {
    fn solve1(&self, input: &str) -> PuzzleResult;
    fn solve2(&self, input: &str) -> PuzzleResult;
}

pub fn display_result(result: PuzzleResult) -> String {
    match result {
        Ok(answer) => answer,
        Err(error) => {
            eprintln!("Error caught: {error:?}\n");
            error.to_string()
        }
    }
}
