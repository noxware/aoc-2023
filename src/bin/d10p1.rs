use aocd::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }

    fn up(&self) -> Self {
        Self {
            y: self.y - 1,
            ..*self
        }
    }

    fn down(&self) -> Self {
        Self {
            y: self.y + 1,
            ..*self
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            ..*self
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            ..*self
        }
    }

    fn neighbors(&self) -> [Self; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    absolute_pos: Pos,
    connections: [Pos; 2],
    raw: char,
}

impl Tile {
    fn from(raw: char, x: i64, y: i64) -> Self {
        let pos = Pos::new(x, y);

        let connections = match raw {
            '|' => [pos.down(), pos.up()],
            '-' => [pos.left(), pos.right()],
            'L' => [pos.up(), pos.right()],
            'J' => [pos.up(), pos.left()],
            '7' => [pos.down(), pos.left()],
            'F' => [pos.down(), pos.right()],
            _ => [pos, pos],
        };

        Self {
            raw,
            connections,
            absolute_pos: pos,
        }
    }
}

fn parse(puzzle: &str) -> HashMap<Pos, Tile> {
    puzzle
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| Tile::from(c, x as i64, y as i64))
        })
        .map(|t| (t.absolute_pos, t))
        .collect()
}

fn solve(puzzle: &str) -> i64 {
    let world = parse(puzzle);

    let mut visited: HashSet<Pos> = HashSet::new();

    let starting_tile = world
        .values()
        .find(|t| t.raw == 'S')
        .expect("no starting tile found");

    visited.insert(starting_tile.absolute_pos);

    let mut current = starting_tile
        .absolute_pos
        .neighbors()
        .iter()
        .filter_map(|p| world.get(p))
        .find(|t| {
            t.connections
                .iter()
                .any(|p| *p == starting_tile.absolute_pos)
        })
        .expect("there is no place to go from starting point");

    loop {
        visited.insert(current.absolute_pos);
        current = current
            .connections
            .iter()
            .filter_map(|p| world.get(p))
            .find(|t| !visited.contains(&t.absolute_pos))
            .unwrap_or(starting_tile);

        if current.raw == 'S' {
            break;
        }
    }

    visited.len() as i64 / 2
}

#[aocd(2023, 10)]
fn main() {
    let input = input!();
    let result = solve(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_against_example() {
        let puzzle: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

        assert_eq!(solve(puzzle), 8);
    }
}
