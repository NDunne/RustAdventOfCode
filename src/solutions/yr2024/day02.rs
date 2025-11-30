use crate::solver::{Solver, Solution, SolutionResult};

/**
 * Sequences that are safe if one element is removed are "Dampened", which stores the index
 * of the removed element for debugging purposes
 */
#[derive(PartialEq, Eq, Debug)]
enum ReportSafety
{
    Unsafe,
    Dampened(usize),
    Safe
}

fn downgrade(input: &ReportSafety, index: &usize) -> ReportSafety
{
    match input {
        ReportSafety::Safe => ReportSafety::Dampened(index.clone()),
        ReportSafety::Dampened(_) => ReportSafety::Unsafe,
        ReportSafety::Unsafe => ReportSafety::Unsafe,
    }
}

const THRESHOLD: i32 = 3;

pub struct SolverDay02 {}

impl SolverDay02
{
    /**
     * A 'Safe' Pair has a difference of at least 1 and at most 3, in the 
     * same direction as the sequence it is contained in
     */
    fn determine_safe_pair(v1: &i32, v2: &i32, direction: &mut i32) -> bool
    {
        let diff = v2 - v1;
        
        let abs_diff = diff.abs();
        let diff_sign = diff.checked_div(abs_diff).unwrap_or(0);

        let mut result = true;
        if abs_diff > THRESHOLD || abs_diff == 0 || diff_sign + *direction == 0
        {
            result = false
        }        
        else
        {
            *direction = diff_sign;
        }
        
        result
    }

    /**
     * Determine safe is called recursively once on the list with just the
     * first element removed, as the direction criteria means it is 
     * undecidable which of the first two elements to remove if they are unsafe
     * without looking ahead in the list
     */
    fn determine_safe_rec(report: &[i32], recurse: bool) -> ReportSafety
    {
        let mut direction = 0;
 
        let mut safety = ReportSafety::Safe;

        let mut va_idx = 0;
        let mut vb_idx = 1;

        if report.len() < 2
        {
            return ReportSafety::Safe;
        }

        while vb_idx < report.len()
        {
            if !Self::determine_safe_pair(&report[va_idx], &report[vb_idx], &mut direction)
            {
                safety = downgrade(&safety, &vb_idx);
                if safety == ReportSafety::Unsafe
                {
                    break;
                }
                if recurse && va_idx < 2
                {
                    let tail = &report[1..];
                    let tail_safety = Self::determine_safe_rec(tail, false);
                    if tail_safety == ReportSafety::Safe
                    {
                        return ReportSafety::Dampened(0);
                    }
                    else if va_idx == 1
                    {
                        direction = 0;
                        va_idx = 0;
                        safety = match tail_safety {
                            ReportSafety::Dampened(idx) => ReportSafety::Dampened(idx + 1),
                            ReportSafety::Unsafe => ReportSafety::Dampened(1),
                            _ => safety 
                        };
                        if tail.len() <= 2 
                        {
                            break
                        }
                        continue;
                    }
                }
            }
            else
            {
                va_idx = vb_idx;
            }
            vb_idx += 1;
        }

        safety
    }

    fn determine_safe(report: &[i32]) -> ReportSafety
    {
        Self::determine_safe_rec(report, true)
    }
}

impl Solver for SolverDay02
{
    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult
    {
        let mut result = Solution::default();

        for line in lines
        {
            let row_items: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

            let line_safety = Self::determine_safe(&row_items);

            result.part1 += (line_safety == ReportSafety::Safe) as isize;
            result.part2 += (line_safety != ReportSafety::Unsafe) as isize;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

     #[test]
    fn test_base_cases()
    {
        assert_eq!(ReportSafety::Safe, SolverDay02::determine_safe(&vec![]));
        assert_eq!(ReportSafety::Safe, SolverDay02::determine_safe(&vec![0]));
        assert_eq!(ReportSafety::Safe, SolverDay02::determine_safe(&vec![0, 1]));
        assert_eq!(ReportSafety::Dampened(0), SolverDay02::determine_safe(&vec![0, 0]));
        assert_eq!(ReportSafety::Dampened(0), SolverDay02::determine_safe(&vec![0, 4]));
    }

    #[test]
    fn test_basic_safe()
    {
        assert_eq!(ReportSafety::Safe, SolverDay02::determine_safe(&vec![1, 2, 3]));
        assert_eq!(ReportSafety::Safe, SolverDay02::determine_safe(&vec![3, 2, 1]));
    }

    #[test]
    fn test_basic_unsafe()
    {
        assert_eq!(ReportSafety::Unsafe, SolverDay02::determine_safe(&vec![1, 5, 9]));
        assert_eq!(ReportSafety::Unsafe, SolverDay02::determine_safe(&vec![9, 5, 1]));
    }

    #[test]
    fn test_first_dampend()
    {
        assert_eq!(ReportSafety::Dampened(0), SolverDay02::determine_safe(&vec![1, 5, 6]));
        assert_eq!(ReportSafety::Dampened(0), SolverDay02::determine_safe(&vec![5, 1, 2]));
        assert_eq!(ReportSafety::Dampened(0), SolverDay02::determine_safe(&vec![1, 6, 5]));
        assert_eq!(ReportSafety::Dampened(0), SolverDay02::determine_safe(&vec![6, 2, 1]));
    }

    #[test]
    fn test_middle_dampend()
    {
        assert_eq!(ReportSafety::Dampened(1), SolverDay02::determine_safe(&vec![1, 6, 2]));
        assert_eq!(ReportSafety::Dampened(1), SolverDay02::determine_safe(&vec![5, 1, 6]));
        assert_eq!(ReportSafety::Dampened(1), SolverDay02::determine_safe(&vec![6, 1, 5]));
        assert_eq!(ReportSafety::Dampened(1), SolverDay02::determine_safe(&vec![2, 6, 1]));
    }

    #[test]
    fn test_last_dampend()
    {
        assert_eq!(ReportSafety::Dampened(2), SolverDay02::determine_safe(&vec![1, 2, 6]));
        assert_eq!(ReportSafety::Dampened(2), SolverDay02::determine_safe(&vec![5, 6, 1]));
        assert_eq!(ReportSafety::Dampened(2), SolverDay02::determine_safe(&vec![6, 5, 1]));
        assert_eq!(ReportSafety::Dampened(2), SolverDay02::determine_safe(&vec![2, 1, 6]));
    }

    #[test]
    fn test_ambiguous_direction_dampend()
    {
        assert_eq!(ReportSafety::Dampened(0), SolverDay02::determine_safe(&vec![6, 9, 6, 3]));
        assert_eq!(ReportSafety::Dampened(1), SolverDay02::determine_safe(&vec![6, 9, 3, 0]));
    }
    
    #[test]
    fn test_sample()
    {
        let sample: &str = "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        ";

        let solution = SolverDay02::solve(Box::new(sample.split('\n'))).unwrap();

        assert_eq!(solution.part1, 2);
        assert_eq!(solution.part2, 4);
    }

    #[test]
    fn test_regression()
    {
        let values: Vec<i32> = "39 41 41 42 44 46 49 46".split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        assert_eq!(ReportSafety::Unsafe, SolverDay02::determine_safe(&values));
    }
}