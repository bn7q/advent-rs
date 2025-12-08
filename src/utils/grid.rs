use std::ops;

type Csize = i64;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Vector {
    x: Csize,
    y: Csize,
}
pub type Point = Vector;

pub const UP: Vector = Vector { x: -1, y: 0 };
pub const DOWN: Vector = Vector { x: 1, y: 0 };
pub const LEFT: Vector = Vector { x: 0, y: -1 };
pub const RIGHT: Vector = Vector { x: 0, y: 1 };

pub const NEIGBOURS: [Vector; 4] = [UP, DOWN, LEFT, RIGHT];
pub const NEIGBOURS_ALL: [Vector; 8] = [
    UP,
    DOWN,
    LEFT,
    RIGHT,
    Vector { x: -1, y: -1 },
    Vector { x: -1, y: 1 },
    Vector { x: 1, y: -1 },
    Vector { x: 1, y: 1 },
];

impl Vector {
    pub fn new(x: Csize, y: Csize) -> Self {
        Self { x: x, y: y }
    }

    // clockwise
    pub fn rotate(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    // counter-clockwise
    pub fn rotate_back(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn neigbours(&self) -> impl Iterator<Item = Self> {
        NEIGBOURS.iter().map(|d| *self + *d)
    }

    // with diagonals
    pub fn neigbours_all(&self) -> impl Iterator<Item = Self> {
        NEIGBOURS_ALL.iter().map(|d| *self + *d)
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

pub struct Grid<T> {
    x_len: usize,
    y_len: usize,
    grid: Vec<Vec<T>>,
}

pub struct GridElement<T> {
    pub point: Point,
    pub value: T,
}

impl<T: Copy> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Grid<T> {
        Grid {
            x_len: data.len(),
            y_len: data[0].len(),
            grid: data,
        }
    }

    pub fn fits_dimensions(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.x_len as i64 && point.y >= 0 && point.y < self.y_len as i64
    }

    pub fn get_neigbours_of(&self, point: &Point) -> impl Iterator<Item = T> {
        point.neigbours().flat_map(|p| self.get(&p))
    }

    pub fn get_all_neigbours_of(&self, point: &Point) -> impl Iterator<Item = T> {
        point.neigbours_all().flat_map(|p| self.get(&p))
    }

    pub fn get(&self, point: &Point) -> Option<T> {
        if self.fits_dimensions(&point) {
            Some(self.grid[point.x as usize][point.y as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, point: &Point, value: T) -> Result<(), Error> {
        if self.fits_dimensions(&point) {
            self.grid[point.x as usize][point.y as usize] = value;
            Ok(())
        } else {
            Err(Error::IndicesOutOfBounds(*point))
        }
    }

    pub fn values(&self) -> impl Iterator<Item = T> {
        (0..self.x_len)
            .flat_map(move |x_idx| (0..self.y_len).map(move |y_idx| self.grid[x_idx][y_idx]))
            .into_iter()
    }

    pub fn elements(&self) -> impl Iterator<Item = GridElement<T>> {
        (0..self.x_len)
            .flat_map(move |x_idx| {
                (0..self.y_len).map(move |y_idx| GridElement {
                    point: Point {
                        x: x_idx as i64,
                        y: y_idx as i64,
                    },
                    value: self.grid[x_idx][y_idx],
                })
            })
            .into_iter()
    }
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| {
                    n.next()
                        .expect("2D array is corrupted, unable to transpose")
                })
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    IndicesOutOfBounds(Point),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IndicesOutOfBounds(point) => write!(f, "Indices {point:?} out of bounds"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Vector3D {
    pub x: Csize,
    pub y: Csize,
    pub z: Csize,
}
pub type Point3D = Vector3D;

impl Vector3D {
    pub fn new(x: Csize, y: Csize, z: Csize) -> Self {
        Self { x, y, z }
    }

    pub fn from_slice(coordinates: [Csize; 3]) -> Self {
        let [x, y, z] = coordinates;
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}
