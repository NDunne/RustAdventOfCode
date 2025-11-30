use std::fmt;

#[derive(Default, Debug)]
pub struct Solution
{
    pub part1: isize,
    pub part2: isize
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Part 1: {} | Part 2: {}", self.part1, self.part2)
    }
}

pub type SolutionResult = anyhow::Result<Solution>;

pub trait Solver {
    fn clean<'a>(lines: Box<dyn Iterator<Item = &'a str> + 'a>) -> Vec<&'a str>
    {
        lines.into_iter().filter_map(|line| {
            match line.trim() {
                l if !l.is_empty() => Some(l),
                _ => None
            }
        }).collect()
    }

    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult;

    fn solve<'a>(lines: Box<dyn Iterator<Item = &'a str> + 'a>) -> SolutionResult
    {
        Self::solve_impl(Self::clean(lines))
    }
}



#[cfg(test)]
mod test
{
    use super::*;

    struct TestSolver {}

    impl Solver for TestSolver
    {
        fn solve_impl<'a>(_lines: Vec<&'a str>) -> SolutionResult
        {
            Ok(Solution::default())
        }

    }

    #[test]
    fn test_clean()
    {
        let input = vec![
"",
"a b c",
"",
"    d e f     ",
""
        ];

        let expected: Vec<&str> = vec!["a b c", "d e f"]; 

        assert_eq!(TestSolver::clean(Box::new(input.into_iter())), expected);
    }
}

