use crate::{
    puzzle::{Puzzle, PuzzleResult},
    utils::split_and_parse,
};

pub struct P;

static PRESENT_BOX_SIDE: usize = 3;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let (presents, trees) = parse_input(input);

        let present_areas: Vec<_> = presents.iter().map(|p| p.area()).collect();

        let res: u32 = trees
            .iter()
            .map(|tree| {
                if tree.max_boxes() >= tree.presents.iter().cloned().sum() {
                    1 // This definitely fits
                } else if tree.space()
                    < tree
                        .presents
                        .iter()
                        .zip(&present_areas)
                        .map(|(p, area)| p * area)
                        .sum()
                {
                    0 // This definitely doesn't
                } else {
                    panic!("Oh no! That's too complcated");
                }
            })
            .sum();

        return Ok(res.to_string());
    }

    fn solve2(&self, _input: &str) -> PuzzleResult {
        return Ok("        ب	 ⃰. ⃰. ⃰. ⃰. ⃰. ⃰.*".to_string());
    }
}

struct Tree {
    x: u32,
    y: u32,
    presents: Vec<u32>,
}

impl Tree {
    fn space(&self) -> u32 {
        self.x * self.y
    }

    fn max_boxes(&self) -> u32 {
        self.x.div_euclid(PRESENT_BOX_SIDE as u32) * self.y.div_euclid(PRESENT_BOX_SIDE as u32)
    }
}

struct Present([[u8; PRESENT_BOX_SIDE]; PRESENT_BOX_SIDE]);

impl Present {
    fn area(&self) -> u32 {
        self.0
            .map(|r| r.iter().filter(|&b| *b == b'#').count() as u32)
            .iter()
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Tree>) {
    let mut lines = input.lines();

    let mut presents = Vec::new();
    let mut trees = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        if line.as_bytes()[1] == b':' {
            let present = (0..PRESENT_BOX_SIDE)
                .map(|_| {
                    lines
                        .next()
                        .unwrap()
                        .as_bytes()
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            presents.push(Present(present));
            continue;
        }

        let [dim, counts]: [String; 2] = split_and_parse(line, ':').unwrap();
        let [x, y] = split_and_parse(dim.as_str(), 'x').unwrap();
        let present_counts = counts
            .trim()
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();
        trees.push(Tree {
            x,
            y,
            presents: present_counts,
        });
    }

    (presents, trees)
}
