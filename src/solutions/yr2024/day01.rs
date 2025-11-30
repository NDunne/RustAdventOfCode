use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use crate::solver::{Solver, Solution, SolutionResult};

pub struct SolverDay01 {}

impl Solver for SolverDay01
{
    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult
    {    
        let mut col1: Vec<isize> = Vec::new();
        let mut col2: Vec<isize> = Vec::new();

        for line in lines
        {
            let cols = line.split_whitespace().collect::<Vec<&str>>();
            if cols.len() != 2
            {
                continue;
            }
            col1.push(cols[0].parse::<isize>()?);
            col2.push(cols[1].parse::<isize>()?);
        }
        col1.sort();
        col2.sort();

        let mut col2_counter = HashMap::new();
        
        for value in col2.iter() {
            match col2_counter.entry(value) {
                Occupied(mut e) => *e.get_mut() += 1,
                Vacant(e) => {
                    e.insert(1);
                }
            }
        }

        Ok(col1
            .iter()
            .zip(col2.iter())
            .fold(Solution::default(), |mut res, (col1_v, col2_v)| {
                res.part1 += (*col1_v - *col2_v).abs();
                res.part2 += *col1_v * col2_counter.get(col1_v).unwrap_or(&0);
                res
            })
        )
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_sample()
    {
        let sample: &str = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        ";

        let solution = SolverDay01::solve(Box::new(sample.split('\n'))).unwrap();

        assert_eq!(solution.part1, 11);
        assert_eq!(solution.part2, 31);
    }
}

