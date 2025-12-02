/// This file is auto generated. Do not modify!
use crate::puzzle::Puzzle;
pub mod y2024 {
    pub mod d01;
}
pub mod y2025 {
    pub mod d00;
    pub mod d01;
    pub mod d02;
    pub mod d03;
}

pub fn get_puzzle(y: u16, d: u16) -> Option<Box<dyn Puzzle>> {
    match (y, d) {
        (2024, 1) => Some(Box::new(y2024::d01::P)),
        (2025, 0) => Some(Box::new(y2025::d00::P)),
        (2025, 1) => Some(Box::new(y2025::d01::P)),
        (2025, 2) => Some(Box::new(y2025::d02::P)),
        (2025, 3) => Some(Box::new(y2025::d03::P)),
        _ => None,
    }
}
