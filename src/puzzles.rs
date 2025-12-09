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
    pub mod d04;
    pub mod d05;
    pub mod d06;
    pub mod d07;
    pub mod d08;
    pub mod d09;
    pub mod d10;
}

pub fn get_puzzle(y: u16, d: u16) -> Option<Box<dyn Puzzle>> {
    match (y, d) {
        (2024, 1) => Some(Box::new(y2024::d01::P)),
        (2025, 0) => Some(Box::new(y2025::d00::P)),
        (2025, 1) => Some(Box::new(y2025::d01::P)),
        (2025, 2) => Some(Box::new(y2025::d02::P)),
        (2025, 3) => Some(Box::new(y2025::d03::P)),
        (2025, 4) => Some(Box::new(y2025::d04::P)),
        (2025, 5) => Some(Box::new(y2025::d05::P)),
        (2025, 6) => Some(Box::new(y2025::d06::P)),
        (2025, 7) => Some(Box::new(y2025::d07::P)),
        (2025, 8) => Some(Box::new(y2025::d08::P)),
        (2025, 9) => Some(Box::new(y2025::d09::P)),
        (2025, 10) => Some(Box::new(y2025::d10::P)),
        _ => None,
    }
}
