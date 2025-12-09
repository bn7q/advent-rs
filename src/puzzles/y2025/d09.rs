use std::cmp::max;

use geo::{Contains, Coord, LineString, Polygon, Rect};

use crate::{
    puzzle::{Puzzle, PuzzleResult},
    utils::{grid::Point, split_and_parse},
};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let points = input
            .lines()
            .map(|l| split_and_parse(l, ',').map(|coord| Point::from_slice(coord)))
            .collect::<Result<Vec<Point>, _>>()?;

        let mut largest_area = 0;

        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let square = rect_area(points[i], points[j]);
                largest_area = max(square, largest_area);
            }
        }

        return Ok(largest_area.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let points = input
            .lines()
            .map(|l| split_and_parse(l, ',').map(|coord| Point::from_slice(coord)))
            .collect::<Result<Vec<Point>, _>>()?;

        let polygon = Polygon::new(
            LineString(points.iter().map(|&v| v.into()).collect()),
            vec![],
        );

        let mut largest_area = 0;

        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let area = rect_area(points[i], points[j]);
                if area > largest_area {
                    let rect = Rect::new(points[i], points[j]);
                    if polygon.contains(&rect) {
                        largest_area = area;
                    }
                }
            }
        }

        return Ok(largest_area.to_string());
    }
}

fn rect_area(p1: Point, p2: Point) -> i64 {
    ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
}

impl Into<Coord> for Point {
    fn into(self) -> Coord {
        Coord {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}
