
use crate::solver::{Solver, Solution, SolutionResult};

use regex::Regex;

pub struct SolverDay03 {}

impl SolverDay03
{
    fn process_chunk(chunk: &str) -> isize
    {
        let mul_matcher = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        
        mul_matcher.captures_iter(chunk).fold(0, |prod_sum, captures|
        {
            let lhs = captures[1].parse::<isize>().unwrap_or(0);
            let rhs = captures[2].parse::<isize>().unwrap_or(0);
            prod_sum + (lhs * rhs)
        })
    }
}

impl Solver for SolverDay03
{
    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult
    {
        let mut result = Solution::default();
        let full_input = lines.join("");

        let mut enabled = true;

        for chunk in full_input.split("do")
        {
            if enabled && chunk.starts_with("n't()")
            {
                enabled = false;
            }
            if !enabled && chunk.starts_with("()")
            {
                enabled = true;
            }

            let chunk_sum = Self::process_chunk(chunk);
            
            result.part1 += chunk_sum;
            if enabled
            {
                result.part2 += chunk_sum
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_sample_part1()
    {
        let sample: &str = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        ";
        
        let solution = SolverDay03::solve(Box::new(sample.split('\n'))).unwrap();
        assert_eq!(solution.part1, 161);
    }

        #[test]
    fn test_sample_part2()
    {
        let sample: &str = "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)
        ";
        
        let solution = SolverDay03::solve(Box::new(sample.split('\n'))).unwrap();
        assert_eq!(solution.part2, 48);
    }
}