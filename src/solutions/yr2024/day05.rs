use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use crate::solver::{Solver, Solution, SolutionResult};

#[derive(Default)]
struct Rule
{
    prohibited_before: HashSet<isize>,
    prohibited_after: HashSet<isize>
}

#[derive(Default)]
struct RuleChecker {
    rules: HashMap<isize, Rule>
}

enum RuleResult {
    Correct(isize),
    ReOrdered(isize)
}

impl RuleChecker
{
    // A 'Rule' implies first must appear only before second if both are present
    fn add_rule(&mut self, first: isize, second: isize)
    {
        self.rules.entry(first).or_insert(Rule::default()).prohibited_before.insert(second);
        self.rules.entry(second).or_insert(Rule::default()).prohibited_after.insert(first);
    }

    // Recursive call adds the prohibited list 
    fn check_rec(&self, update_slice: &[isize], prohibited: HashSet<isize>) -> Option<isize>
    {
        let pivot_idx = update_slice.len()/2;
        let pivot =  update_slice[pivot_idx];

        if prohibited.contains(&pivot)
        {
            return None;
        }

        let left = &update_slice[..pivot_idx];
        let mut left_prohibited = prohibited.clone();

        let right = &update_slice[pivot_idx+1..];
        let mut right_prohibited = prohibited;
        
        if let Some(pivot_rule) = self.rules.get(&pivot)
        {
            left_prohibited.extend(&pivot_rule.prohibited_before);
            right_prohibited.extend(&pivot_rule.prohibited_after);
        }

        if (left.len() == 0 || self.check_rec(left, left_prohibited).is_some()) &&
            (right.len() == 0 || self.check_rec(right, right_prohibited).is_some())
            {
                return Some(pivot);
            }
        None
    }

    // Search the provided string as a tree, rooted at the middle element. Returns the middle element
    fn check(&self, update: &Vec<isize>) -> RuleResult
    {
        if let Some(middle_val) = self.check_rec(update, HashSet::new())
        {
            return RuleResult::Correct(middle_val);
        }

        let re_ordered = self.correct(&update);
        RuleResult::ReOrdered(re_ordered[re_ordered.len()/2])
    }

    // Comparison function used to correct ordering
    fn get_order(&self, left: &isize, right: &isize) -> Ordering
    {
        if let Some(pivot_rule) = self.rules.get(&left)
        {
            if pivot_rule.prohibited_before.contains(&right)
            {
                return Ordering::Less;
            }
            else if pivot_rule.prohibited_after.contains(&right)
            {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }

    // Use the rules to fix an incorrectly ordered update
    fn correct(&self, update: &Vec<isize>) -> Vec<isize>
    {
        let mut re_ordered = update.clone();
        re_ordered.sort_by(|x, y| self.get_order(x, y));
        re_ordered
    }
}

pub struct SolverDay05 {}

impl Solver for SolverDay05
{
    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult
    {
        let mut result = Solution::default();

        let mut rule_checker =  RuleChecker::default();

        for line in lines
        {
            if line.contains('|')
            {
                let rule_parts: Vec<isize> = line.split('|').map(|x| x.parse::<isize>().unwrap()).collect();
                rule_checker.add_rule(rule_parts[0], rule_parts[1]);
                continue;              
            }

            let update_parts: Vec<isize> = line.split(',').map(|x| x.parse::<isize>().unwrap()).collect();

            // Sums middle elements depending on if ordering was correct
            match rule_checker.check(&update_parts)
            {
                RuleResult::Correct(value) => result.part1 += value,
                RuleResult::ReOrdered(value) => result.part2 += value
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
    fn test_sort()
    {
        let list: Vec<isize> = vec![1, 2, 3, 4];

        let mut rule_checker = RuleChecker::default();

        rule_checker.add_rule(2, 1);

        assert_eq!(rule_checker.correct(&list), vec![2, 1, 3, 4]);
    }

    #[test]
    fn test_sample()
    {
        let sample: &str = "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47";
    
        let solution = SolverDay05::solve(Box::new(sample.split('\n'))).unwrap();
        assert_eq!(solution.part1, 143);
        assert_eq!(solution.part2, 123);

    }

}
