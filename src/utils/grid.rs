#![allow(dead_code)]
use std::ops;

type Csize = i64;
struct Vector {
    x: Csize,
    y: Csize,
}
type Point = Vector;

const UP: Vector = Vector { x: -1, y: 0 };
const DOWN: Vector = Vector { x: 1, y: 0 };
const LEFT: Vector = Vector { x: 0, y: -1 };
const RIGHT: Vector = Vector { x: 0, y: 1 };

impl Vector {
    fn new(x: Csize, y: Csize) -> Self {
        Self { x: x, y: y }
    }

    // clockwise
    fn rotate(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    // counter-clockwise
    fn rotate_back(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    fn neigbours(self) -> [Self; 4] {
        [
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ]
    }

    // with diagonals
    fn neigbours_all(self) -> [Self; 8] {
        [
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y - 1),
            Self::new(self.x - 1, self.y + 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x + 1, self.y + 1),
        ]
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}
