mod yr2024;
mod yr2025;

use crate::solver::{Solver, SolutionResult};

pub fn solve<'a>(year: &str, day_number : u8, lines: Box<dyn Iterator<Item = &'a str> + 'a>) -> SolutionResult
{
    match year {
        "2024" => match day_number {
                1 => yr2024::day01::SolverDay01::solve(lines),
                2 => yr2024::day02::SolverDay02::solve(lines),
                3 => yr2024::day03::SolverDay03::solve(lines),
                4 => yr2024::day04::SolverDay04::solve(lines),
                5 => yr2024::day05::SolverDay05::solve(lines),
                6 => yr2024::day06::SolverDay06::solve(lines),
                _ => Err(anyhow::anyhow!("{} day {} Not Implemented", year, day_number))
        },
        "2025" => match day_number {
                _ => Err(anyhow::anyhow!("{} day {} Not Implemented", year, day_number))

        }
        _ => Err(anyhow::anyhow!(format!("{} Not Implemented", year)))
    }
}