use advent_rs::{puzzle::display_result, puzzles, utils};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <year> <day> [--test]", args[0]);
        eprintln!("Example: {} 2025 1", args[0]);
        eprintln!("Example: {} 2025 1 --test", args[0]);
        std::process::exit(1);
    }

    let year: u16 = args[1].parse().expect("Year must be a number");
    let day: u16 = args[2].parse().expect("Day must be a number");
    let is_test = args.len() > 3 && args[3] == "--test";

    // Get the puzzle
    let puzzle = puzzles::get_puzzle(year, day).unwrap_or_else(|| {
        eprintln!("Puzzle for year {} day {} not found", year, day);
        std::process::exit(1);
    });

    // Read input
    let input = utils::read_input(year, day, is_test).unwrap_or_else(|e| {
        eprintln!("Failed to read input: {}", e);
        std::process::exit(1);
    });

    println!(
        "=== Year {} - Day {} {} ===",
        year,
        day,
        if is_test { "(TEST)" } else { "" }
    );

    let (result1, time1) = with_duration!(puzzle.solve1(input.as_str()));
    println!("Part 1: {}", display_result(result1));
    println!("Took:   {:?}\n", time1);

    let (result2, time2) = with_duration!(puzzle.solve2(input.as_str()));
    println!("Part 2: {}", display_result(result2));
    println!("Took:   {:?}\n", time2);
}

#[macro_export]
macro_rules! with_duration {
    ($func:expr) => {{
        let start = std::time::Instant::now();
        let result = ($func);
        (result, start.elapsed())
    }};
}
