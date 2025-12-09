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
                let square = ((points[i].x - points[j].x).abs() + 1) * ((points[i].y - points[j].y).abs() + 1);
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
            LineString(
                points
                    .iter()
                    .map(|v| Coord {
                        x: v.x as f64,
                        y: v.y as f64,
                    })
                    .collect(),
            ),
            vec![],
        );

        let mut largest_area = 0;

        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let area = ((points[i].x - points[j].x).abs() + 1) * ((points[i].y - points[j].y).abs() + 1);
                if area > largest_area {
                    let rect = Rect::new(
                        Coord {
                            x: points[i].x as f64,
                            y: points[i].y as f64,
                        },
                        Coord {
                            x: points[j].x as f64,
                            y: points[j].y as f64,
                        },
                    );
                    if polygon.contains(&rect) {
                        largest_area = area;
                    }
                }
            }
        }

        return Ok(largest_area.to_string());
    }
}
