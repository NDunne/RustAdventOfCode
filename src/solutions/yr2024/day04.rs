use std::{collections::HashMap, slice::Iter};
use std::fmt;
use std::hash::Hash;

use itertools::Itertools;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::solver::{Solver, Solution, SolutionResult};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point
{
    x: usize,
    y: usize
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq)]
enum Direction
{
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7
}

impl Direction
{
    fn is_perp(&self, other: &Direction) -> bool
    {
        i8::abs((*self as i8) - (*other as i8)) % 4 == 2
    }
}

impl Point
{
    fn get_relative(&self, direction: &Direction, count: usize) -> Option<Point>
    {
        match direction {
            Direction::E  => Some(Point { x: self.x.checked_add(count)?, y: self.y                     }),
            Direction::SE => Some(Point { x: self.x.checked_add(count)?, y: self.y.checked_add(count)? }),
            Direction::S  => Some(Point { x: self.x,                     y: self.y.checked_add(count)? }),
            Direction::SW => Some(Point { x: self.x.checked_sub(count)?, y: self.y.checked_add(count)? }),
            Direction::W  => Some(Point { x: self.x.checked_sub(count)?, y: self.y                     }),
            Direction::NW => Some(Point { x: self.x.checked_sub(count)?, y: self.y.checked_sub(count)? }),
            Direction::N  => Some(Point { x: self.x,                     y: self.y.checked_sub(count)? }),
            Direction::NE => Some(Point { x: self.x.checked_add(count)?, y: self.y.checked_sub(count)? }),
        }
    }
}


struct LetterGrid<'a>
{
    grid: Vec<&'a str>
}

impl<'a> LetterGrid<'a>
{
    fn new(grid: Vec<&'a str>) -> Self
    {
        Self { grid }
    }

    fn iter(&'a self) -> Iter<'a, &'a str>
    {
        self.grid.iter()
    }

    fn get(&'a self, pos: &Point)  -> Option<char>
    {
        Some(self.grid.get(pos.y)?.chars().nth(pos.x)?)
    }

    fn find_all_char(&'a self, needle: &char) -> Vec<Point>
    {
        self.iter().enumerate().fold(Vec::<Point>::new(), |mut points, (row, s)| {
            points.extend(s.match_indices(*needle).map(|(idx, _)| Point { x: idx, y: row}));
            points
        })
    }

    fn find_all_word(&self, word: &str) -> Vec<(Point, Direction)>
    {
        let candidate_origins = self.find_all_char( &word.chars().nth(0).unwrap());

        candidate_origins.iter().cartesian_product(Direction::iter()).filter_map(|(origin, direction)| {
            let mut next_point = origin.clone();

            for c in word[1..].chars()
            {
                next_point = next_point.get_relative(&direction,1)?;
                self.get(&next_point).filter(|n| *n == c)?;
            }
            Some((origin.clone(), direction.clone()))
        }).collect()
    }

    fn find_all_x_word(&self, word: &str) -> Vec<(Point, (Direction, Direction))>
    {
        let words = self.find_all_word(word);
        let center_dist = word.len() >> 1;

        let diagonal_words_by_center = words.iter().fold(HashMap::new(), |mut map, (start, direction)| {
            if let Direction::NE | Direction::SE | Direction::SW | Direction::NW = *direction
            {
                let center = start.get_relative(direction, center_dist).unwrap();
                map.entry(center).or_insert_with(|| Vec::new()).push(direction.clone());
            }
            map
        });

        let mut results = Vec::new();
        for (center, directions) in diagonal_words_by_center
        {
            for pair in directions.iter().combinations(2)
            {
                if pair[0].is_perp(pair[1])
                {
                    let x_descriptor = (center, (*pair[0], *pair[1]));
                    results.push(x_descriptor);
                }
            }
        }

        results
    }
}

impl<'a> fmt::Display for LetterGrid<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter().join("\n"))
    }
}

pub struct SolverDay04 {}

impl Solver for SolverDay04
{
    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult
    {
        let wordsearch = LetterGrid::new(lines);
        let word = "XMAS";
       
        let mut result = Solution::default();
        result.part1 = wordsearch.find_all_word(word).len() as isize;

        let x_word = "MAS";
        result.part2 = wordsearch.find_all_x_word(x_word).len() as isize;
        
        Ok(result)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_point_get_neighbour()
    {
        let start = Point { x: 5, y: 7 };
        assert_eq!(start.get_relative(&Direction::N,  1),  Some(Point { x: 5, y: 6}));
        assert_eq!(start.get_relative(&Direction::NE, 1), Some(Point { x: 6, y: 6}));
        assert_eq!(start.get_relative(&Direction::E,  1),  Some(Point { x: 6, y: 7}));
        assert_eq!(start.get_relative(&Direction::SE, 1), Some(Point { x: 6, y: 8}));
        assert_eq!(start.get_relative(&Direction::S,  1),  Some(Point { x: 5, y: 8}));
        assert_eq!(start.get_relative(&Direction::SW, 1), Some(Point { x: 4, y: 8}));
        assert_eq!(start.get_relative(&Direction::W,  1),  Some(Point { x: 4, y: 7}));
        assert_eq!(start.get_relative(&Direction::NW, 1), Some(Point { x: 4, y: 6}));
        
        let origin = Point { x: 0, y: 0 };
        assert_eq!(origin.get_relative(&Direction::SW, 1), None);
        assert_eq!(origin.get_relative(&Direction::W,  1), None); 
        assert_eq!(origin.get_relative(&Direction::NW, 1), None);
        assert_eq!(origin.get_relative(&Direction::N,  1), None); 
        assert_eq!(origin.get_relative(&Direction::NE, 1), None);
    }

    #[test]
    fn test_point_is_perp()
    {
        assert_eq!(Direction::N.is_perp(&Direction::E), true);
        assert_eq!(Direction::N.is_perp(&Direction::W), true);
        assert_eq!(Direction::N.is_perp(&Direction::NE), false);
        assert_eq!(Direction::N.is_perp(&Direction::NW), false);
    }

    #[test]
    fn test_find_all()
    {
        let input = LetterGrid::new(vec!["X..", ".X.", "..X"]);
    
        assert_eq!(input.find_all_char(&'X'), vec![Point { x: 0, y: 0 }, 
                                              Point { x: 1, y: 1 }, 
                                              Point { x: 2, y: 2}]);
    }

    #[test]
    fn test_grid_get()
    {
        let input = LetterGrid::new(vec!["ABC", "DEF", "GHI"]);
    
        assert_eq!(input.get(&Point { x: 0, y: 1 }).unwrap(), 'D');
        assert_eq!(input.get(&Point { x: 1, y: 0 }).unwrap(), 'B');
        assert_eq!(input.get(&Point { x: 1, y: 1 }).unwrap(), 'E');
        assert_eq!(input.get(&Point { x: 3, y: 3 }), None);
    }

    #[test]
    fn test_find_all_word()
    {
        let grid = LetterGrid::new(vec!["BBB", "BAB", "BBB"]);
        assert_eq!(grid.find_all_word("AB"), vec![
            (Point { x: 1, y: 1}, Direction::N), 
            (Point { x: 1, y: 1}, Direction::NE), 
            (Point { x: 1, y: 1}, Direction::E), 
            (Point { x: 1, y: 1}, Direction::SE), 
            (Point { x: 1, y: 1}, Direction::S), 
            (Point { x: 1, y: 1}, Direction::SW), 
            (Point { x: 1, y: 1}, Direction::W), 
            (Point { x: 1, y: 1}, Direction::NW), 
        ]);
    }

    #[test]
    fn test_find_all_x_word()
    {
        let grid = LetterGrid::new(vec![
            "AAA", 
            "ABC", 
            "CCC"
        ]);
        assert_eq!(grid.find_all_x_word("ABC"), vec![
            (Point { x: 1, y: 1 }, (Direction::SE, Direction::SW)), 
        ]);

        let grid2 = LetterGrid::new(vec![
            "CCC", 
            "CBA", 
            "AAA"
        ]);
        assert_eq!(grid2.find_all_x_word("ABC"), vec![
            (Point { x: 1, y: 1 }, (Direction::NE, Direction::NW))
        ]);

        let grid3 = LetterGrid::new(vec![
            "ACC", 
            "ABC", 
            "AAC"
        ]);
        assert_eq!(grid3.find_all_x_word("ABC"), vec![
            (Point { x: 1, y: 1 }, (Direction::SE, Direction::NE)), 
        ]);

        let grid4 = LetterGrid::new(vec![
            "CAA", 
            "CBA", 
            "CCA"
        ]);
        assert_eq!(grid4.find_all_x_word("ABC"), vec![
            (Point { x: 1, y: 1 }, (Direction::SW, Direction::NW))
        ]);

        let grid5 = LetterGrid::new(vec![
            "AA.", 
            ".B.", 
            ".CC"
        ]);
        assert_eq!(grid5.find_all_x_word("ABC"), vec![]);
    }


    #[test]
    fn test_sample()
    {
        let sample: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        ";

        let solution = SolverDay04::solve(Box::new(sample.split('\n'))).unwrap();
        assert_eq!(solution.part1, 18);
        assert_eq!(solution.part2, 9);
    }

}