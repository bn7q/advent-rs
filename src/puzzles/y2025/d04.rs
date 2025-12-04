use crate::{
    puzzle::{Puzzle, PuzzleResult},
    utils::grid::{Grid, Point},
};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let data = input.lines().map(|l| l.chars().collect()).collect();
        let mut grid = Grid::new(data);

        let res = remove_rolls(&mut grid);

        return Ok(res.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let data = input.lines().map(|l| l.chars().collect()).collect();
        let mut grid = Grid::new(data);

        let mut res = 0;

        while let removed_rolls = remove_rolls(&mut grid)
            && removed_rolls > 0
        {
            res += removed_rolls
        }

        return Ok(res.to_string());
    }
}

fn remove_rolls(grid: &mut Grid<char>) -> usize {
    let removable_rolls: Vec<Point> = grid
        .elements()
        .flat_map(|e| {
            if e.value == '@' {
                let neigbours = grid
                    .get_all_neigbours_of(&e.point)
                    .filter(|&n| n == '@')
                    .count();
                if neigbours < 4 {
                    Some(e.point.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let count = removable_rolls.len();
    for roll in removable_rolls {
        let r = grid.set(&roll, '.');
        let _ = r.inspect_err(|e| eprintln!("{e}"));
    }

    count
}
