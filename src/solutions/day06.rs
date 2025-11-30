use std::{fmt, hash::Hash};
use std::collections::HashSet;

use crate::shared::{Solver, Solution, SolutionResult};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    N,
    E,
    S,
    W
}

impl Direction
{
    fn get_parts(&self) -> (isize, isize)
    {
        match self {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0)
        }
    }
    
    fn right(&self) -> Direction
    {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N
        }
    }
    
    fn opposite(&self) -> Direction
    {
        self.right().right()
    }
    
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", match self
        {
            Direction::S => 'V',
            Direction::E => '>',
            Direction::N => '^',
            Direction::W => '<',
        })
    }
}

#[derive(Clone, Debug)]
enum TileState {
    Clear(HashSet<Direction>),
    Obstacle
}

pub struct Guard {
    position: (isize, isize),
    direction: Direction,
}

impl Guard
{ 
    fn new(position: (isize, isize)) -> Self
    {
        Self { position, direction: Direction::N }
    }

    fn peek_forward(&self) -> (isize, isize)
    {
        let direction_parts = self.direction.get_parts();
        (self.position.0 + direction_parts.0, self.position.1 + direction_parts.1)
    }
    
    fn step_forward(&mut self)
    {
        self.position = self.peek_forward();
    }

    fn step_back(&mut self)
    {
        let direction_parts = self.direction.opposite().get_parts();
        self.position = (self.position.0 + direction_parts.0, self.position.1 + direction_parts.1);
    }
    
    fn peek_right(&self) -> (isize, isize)
    {
        let direction_parts = self.direction.right().get_parts();
        (self.position.0 + direction_parts.0, self.position.1 + direction_parts.1)
    }
    
    fn rotate(&mut self)
    {
        self.direction = self.direction.right();
    }

    fn next(&mut self, current_tile: &TileState)
    {
        if let TileState::Obstacle = current_tile
        {
            self.step_back();
            self.rotate();
        }
        else
        {
            self.step_forward(); 
        }
    }
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.direction)
    }
}


pub struct Map {
    tiles: Vec<TileState>,
    guard: Guard,
    width: usize,
    height: usize,
    visited: isize,
    loop_insert_count: isize
}

impl Map {
    fn new<'a>(lines: Vec<&'a str>) -> Self
    {
        let width = lines[0].len();
        let height = lines.len();

        let mut tiles = vec![TileState::Clear(HashSet::new()); width * height];

        let mut guard_pos: (isize, isize) = (0, 0);

        for (y, row) in lines.iter().enumerate()
        {
            for (x, tile) in row.chars().enumerate()
            {
                match tile
                {
                    '#' => tiles[(y * width ) + x] = TileState::Obstacle,
                    '^' => guard_pos = (x as isize, y as isize),
                    _ => continue
                }
            }
        }

        Self {tiles, guard: Guard::new(guard_pos), width, height, visited: 0, loop_insert_count: 0}
    }

    fn flat(&self, pos: (isize, isize)) -> Option<isize>
    {
        let x = pos.0;
        let y = pos.1;

        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize
        {
            return None;
        }

        Some((y * self.width as isize) + x)
    }

    fn unflat(&self, idx: usize) -> (isize, isize)
    {
        ((idx % self.width) as isize, (idx / self.width) as isize)
    }
}

impl Iterator for Map {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {

        let flat_guard_pos = self.flat(self.guard.position)? as usize;

        if let TileState::Clear(ref mut directions) = self.tiles[flat_guard_pos]
        {
            if directions.len() == 0
            {
                self.visited += 1;
            }
            directions.insert(self.guard.direction);

            if let Some(forward_tile_flat_pos) = self.flat(self.guard.peek_forward())
            {
                if let TileState::Clear(_) = self.tiles[forward_tile_flat_pos as usize]
                {
                    if let Some(right_tile_flat_pos) = self.flat(self.guard.peek_right())
                    {
                        if let TileState::Clear(ref directions) = self.tiles[right_tile_flat_pos as usize]
                        {
                            if directions.contains(&self.guard.direction.right())
                            {
                                self.loop_insert_count += 1;
                            }
                        }
                    }
                }
            }
        }

        self.guard.next(&self.tiles[flat_guard_pos]);
        Some((self.visited, 0))
    }
}


impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    
        let mut repr = String::new();
        for (i, tile) in self.tiles.iter().enumerate()
        {
            if i > 0 && i % self.width == 0
            {
                repr.push('\n');
            }

            if self.unflat(i) == self.guard.position
            {
                repr.push_str(self.guard.to_string().as_str());
                continue;
            }

            repr.push(match tile {
                TileState::Clear(directions) => if directions.len() > 0 { 'X' } else { '.' },
                TileState::Obstacle => '#'
            })
        }

        write!(f, "{}", repr)
    }
}


pub struct SolverDay06 {}

impl Solver for SolverDay06
{
    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult
    {
        let mut result = Solution::default();

        let map = Map::new(lines);

        (result.part1, result.part2) = map.last().unwrap_or((0, 0));

        Ok(result)
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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    
    let solution = SolverDay06::solve(Box::new(sample.split('\n'))).unwrap();
    assert_eq!(solution.part1, 41);
    // assert_eq!(solution.part2, 6);
    }
}