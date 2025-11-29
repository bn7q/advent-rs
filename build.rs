use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let puzzles_file = File::create("src/puzzles.rs").unwrap();

    let (years, days_in_years) = find_puzzles().unwrap();

    let _ = writeln!(
        &puzzles_file,
        "/// This file is auto generated. Do not modify!"
    );
    let _ = writeln!(&puzzles_file, "use crate::puzzle::Puzzle;");

    for (year, days) in years.iter().zip(&days_in_years) {
        let _ = writeln!(&puzzles_file, "pub mod y{} {{", year);

        for day in days {
            let _ = writeln!(&puzzles_file, "    pub mod d{:02};", day);
        }

        let _ = writeln!(&puzzles_file, "}}");
    }

    let _ = writeln!(&puzzles_file, "");

    let _ = writeln!(
        &puzzles_file,
        "pub fn get_puzzle(y: u16, d: u16) -> Option<Box<dyn Puzzle>> {{"
    );
    let _ = writeln!(&puzzles_file, "    match (y, d) {{");

    for (year, days) in years.iter().zip(&days_in_years) {
        for day in days {
            let _ = writeln!(
                &puzzles_file,
                "        ({year}, {day}) => Some(Box::new(y{year}::d{day:02}::P)),"
            );
        }
    }

    let _ = writeln!(&puzzles_file, "        _ => None,");
    let _ = writeln!(&puzzles_file, "    }}");
    let _ = writeln!(&puzzles_file, "}}");

    println!("cargo::rerun-if-changed=build.rs");
}

fn find_puzzles() -> Option<(Vec<u16>, Vec<Vec<u16>>)> {
    let year_re = Regex::new(r"^y([0-9]{4})$").ok()?;
    let day_re = Regex::new(r"^d([0-9]+).rs$").ok()?;

    let mut years = Vec::new();
    let mut days = Vec::new();

    let mut dirs: Vec<fs::DirEntry> = fs::read_dir(Path::new("src/puzzles")).ok()?.flat_map(|f| f.ok()).collect();
    dirs.sort_by_key(|d| d.file_name());

    for dir in dirs {
        let year_path = dir.path();
        let Some(year_captures) = year_re.captures(year_path.file_name()?.to_str()?) else {
            continue;
        };
        let year = year_captures[1].parse::<u16>().ok()?;
        if year_path.is_dir() {
            let mut days_in_year = Vec::new();
            for file in fs::read_dir(year_path).ok()? {
                let file_name = file.ok()?.file_name();
                let Some(day_capture) = day_re.captures(file_name.to_str()?) else {
                    continue;
                };
                days_in_year.push(day_capture[1].parse::<u16>().ok()?);
            }
            days_in_year.sort();
            years.push(year);
            days.push(days_in_year);
        }
    }

    return Some((years, days));
}
