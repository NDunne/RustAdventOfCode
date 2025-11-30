use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::solver::SolutionResult;
use crate::solutions;

pub fn get_solution(year : &str, day_number : u8) -> SolutionResult
{
    let input_filepath = PathBuf::from(format!("./input/{year}/day{:02}.txt", day_number));

    let mut lines: Vec<String> = Vec::new();

    if let Ok(file) = File::open(&input_filepath)
    {
        lines = BufReader::new(file).lines().filter_map(Result::ok).collect();
    }
    let lines_iter = lines.iter().map(|s| s.as_str());
    solutions::solve(year, day_number, Box::new(lines_iter))
}