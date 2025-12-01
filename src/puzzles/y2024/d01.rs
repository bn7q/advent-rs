use crate::{
    puzzle::{Puzzle, PuzzleResult},
    utils::{count_elements, read_grid_parsed},
};

pub struct P;

impl Puzzle for P {
    fn solve1(&self, input: &str) -> PuzzleResult {
        let grid = read_grid_parsed::<i32>(input, 0)?;

        let (mut list1, mut list2): (Vec<i32>, Vec<i32>) =
            grid.iter().map(|v| (v[0], v[1])).unzip();

        list1.sort();
        list2.sort();

        let result: i32 = list1.iter().zip(list2).map(|(l, r)| (l - r).abs()).sum();

        return Ok(result.to_string());
    }

    fn solve2(&self, input: &str) -> PuzzleResult {
        let grid = read_grid_parsed::<u64>(input, 0)?;

        let (list1, list2): (Vec<u64>, Vec<u64>) = grid.iter().map(|v| (v[0], v[1])).unzip();

        let map1 = count_elements(list1);
        let map2 = count_elements(list2);

        let result: u64 = map1
            .iter()
            .map(|(key, count1)| {
                map2.get(key).unwrap_or(&0) * key * count1
            })
            .sum();

        return Ok(result.to_string());
    }
}
