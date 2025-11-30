
use crate::runner::get_solution;
use crate::solver::Solution;

use super::*;
use matches::assert_matches;

#[test]
fn day01()
{
    assert_matches!(get_solution("2024", 1), Ok(Solution { part1: 1830467, part2: 26674158 }));
}

#[test]
fn day02()
{
    assert_matches!(get_solution("2024", 2), Ok(Solution { part1: 390, part2: 439 }));
}

#[test]
fn day03()
{
    assert_matches!(get_solution("2024", 3), Ok(Solution { part1: 187825547, part2: 85508223 }));
}

#[test]
fn day04()
{
    assert_matches!(get_solution("2024", 4), Ok(Solution { part1: 2578, part2: 1972 }));
}

#[test]
fn day05()
{
    assert_matches!(get_solution("2024", 5), Ok(Solution { part1: 4569, part2: 6456 }));
}

#[test]
fn day06()
{
    assert_matches!(get_solution("2024", 6), Ok(Solution { part1: 5312, part2: 0 }));
}
