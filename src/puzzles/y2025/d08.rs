use std::{collections::BTreeSet, num::ParseIntError};

use crate::{
    puzzle::{Puzzle, PuzzleResult},
    utils::{grid::Point3D, split_and_parse},
};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let boxes = input
            .lines()
            .map(|line| split_and_parse(line, ',').map(|coord| Point3D::from_slice(coord)))
            .collect::<Result<Vec<Point3D>, ParseIntError>>()?;
        let boxes_count = boxes.len();
        let mut edges = Vec::new();

        let mut circuits: Vec<BTreeSet<usize>> = (0..boxes_count)
            .map(|b_id| {
                let mut set = BTreeSet::new();
                set.insert(b_id);
                set
            })
            .collect();
        let mut box_in_circuit: Vec<usize> = (0..boxes_count).collect();

        for i in 0..boxes_count {
            for j in (i + 1)..boxes_count {
                edges.push(Edge::from_boxes(i, &boxes[i], j, &boxes[j]));
            }
        }

        edges.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());

        let iterations = if boxes_count == 20 { 10 } else { 1000 };
        for edge in edges.iter().take(iterations) {
            let circuit_a = box_in_circuit[edge.a];
            let circuit_b = box_in_circuit[edge.b];
            if circuit_a == circuit_b {
                continue;
            } else {
                for box_id in circuits[circuit_b].clone() {
                    box_in_circuit[box_id] = circuit_a;
                }
                let mut boxes_in_a = circuits[circuit_b].clone();
                circuits[circuit_b].clear();
                circuits[circuit_a].append(&mut boxes_in_a);
            }
        }

        let mut circuit_sizes: Vec<_> = circuits
            .iter()
            .flat_map(|c| {
                if c.is_empty() {
                    None
                } else {
                    Some(c.len() as u64)
                }
            })
            .collect();

        circuit_sizes.sort();

        let res = circuit_sizes.iter().rev().take(3).fold(1, |a, &b| a * b);

        return Ok(res.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let boxes = input
            .lines()
            .map(|line| split_and_parse(line, ',').map(|coord| Point3D::from_slice(coord)))
            .collect::<Result<Vec<Point3D>, ParseIntError>>()?;
        let boxes_count = boxes.len();
        let mut edges = Vec::new();

        let mut circuits: Vec<BTreeSet<usize>> = (0..boxes_count)
            .map(|b_id| {
                let mut set = BTreeSet::new();
                set.insert(b_id);
                set
            })
            .collect();
        let mut box_in_circuit: Vec<usize> = (0..boxes_count).collect();

        for i in 0..boxes_count {
            for j in (i + 1)..boxes_count {
                edges.push(Edge::from_boxes(i, &boxes[i], j, &boxes[j]));
            }
        }

        edges.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());

        for edge in edges.iter() {
            let circuit_a = box_in_circuit[edge.a];
            let circuit_b = box_in_circuit[edge.b];
            if circuit_a == circuit_b {
                continue;
            } else {
                for box_id in circuits[circuit_b].clone() {
                    box_in_circuit[box_id] = circuit_a;
                }
                let mut boxes_in_a = circuits[circuit_b].clone();
                circuits[circuit_b].clear();
                circuits[circuit_a].append(&mut boxes_in_a);

                if circuits[circuit_a].len() == boxes_count {
                    let dist = boxes[edge.a].x * boxes[edge.b].x;
                    return Ok(dist.to_string());
                }
            }
        }

        return Ok("Solution not found".to_string());
    }
}

struct Edge {
    a: usize,
    b: usize,
    length: f64,
}

impl Edge {
    fn from_boxes(a: usize, a_p: &Point3D, b: usize, b_p: &Point3D) -> Self {
        Edge {
            a,
            b,
            length: a_p.distance_to(b_p),
        }
    }
}
