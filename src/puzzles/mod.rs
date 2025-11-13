use crate::puzzle::Puzzle;

pub mod y2024;

pub fn get_puzzle(y: u16, d:u16) -> Option<Box<dyn Puzzle>> {
    match (y, d) {
        (2024, 1) => Some(Box::new(y2024::d01::P)),
        _ => None
    }
}