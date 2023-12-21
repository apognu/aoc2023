use std::{collections::HashSet, ops::Add};

use itertools::Itertools;
use rayon::prelude::*;

use crate::util::{self, Options};

crate::tests!(16, (46, 51));

enum Tile {
  Empty,
  Horizontal,
  Vertical,
  Slash,
  Backslash,
}

impl From<char> for Tile {
  fn from(symbol: char) -> Self {
    use Tile::*;

    match symbol {
      '.' => Empty,
      '-' => Horizontal,
      '|' => Vertical,
      '/' => Slash,
      '\\' => Backslash,
      _ => panic!("unexpected symbol"),
    }
  }
}

impl Tile {
  fn refract(&self, ((y, x), direction): Move) -> Vec<Move> {
    use Direction::*;
    use Tile::*;

    let direction = match (self, direction) {
      (Empty, direction) => vec![direction],
      (Vertical, Left) | (Vertical, Right) => vec![Up, Down],
      (Horizontal, Up) | (Horizontal, Down) => vec![Left, Right],
      (Horizontal, direction) | (Vertical, direction) => vec![direction],
      (Slash, direction) => vec![direction.refract_slash()],
      (Backslash, direction) => vec![direction.refract_backslash()],
    };

    direction.into_iter().map(|direction| ((y, x) + direction, direction)).collect()
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
  Up,
  Right,
  Left,
  Down,
}

type Grid = Vec<Vec<Tile>>;
type Coord = (isize, isize);
type Move = (Coord, Direction);
type Visited = HashSet<Move>;

impl Add<Direction> for Coord {
  type Output = Coord;

  fn add(self, direction: Direction) -> Self::Output {
    use Direction::*;

    let (y, x) = self;

    match direction {
      Up => (y - 1, x),
      Left => (y, x - 1),
      Right => (y, x + 1),
      Down => (y + 1, x),
    }
  }
}

impl Direction {
  fn refract_backslash(&self) -> Self {
    use Direction::*;

    match self {
      Up => Left,
      Left => Up,
      Right => Down,
      Down => Right,
    }
  }

  fn refract_slash(&self) -> Self {
    use Direction::*;

    match self {
      Up => Right,
      Left => Down,
      Right => Up,
      Down => Left,
    }
  }
}

fn parse_grid(input: &str) -> Grid {
  let input = util::read_file_lines(input);

  input.iter().map(|line| line.chars().map(Tile::from).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn walk(grid: &Grid, mut seen: HashSet<Move>, moveset: Move) -> HashSet<Move> {
  let ((y, x), _) = moveset;

  match grid.get(y as usize).and_then(|row| row.get(x as usize)) {
    None => seen,
    Some(tile) => {
      seen.insert(moveset);

      for moveset in tile.refract(moveset) {
        if !seen.contains(&moveset) {
          seen = walk(grid, seen, moveset);
        }
      }

      seen
    }
  }
}

fn count_energized(energized: HashSet<Move>) -> i64 {
  energized.iter().map(|(coord, _)| coord).collect::<HashSet<_>>().len() as i64
}

fn border_start_moves(grid: &Grid) -> Vec<Move> {
  use Direction::*;

  let width = grid.len() - 1;
  let height = grid[0].len() - 1;

  (0..=height)
    .cartesian_product(0..=height)
    .flat_map(|(row, col)| {
      match (col, row) {
        // Top-left corner
        (0, 0) => vec![((0, 0), Down), ((0, 0), Right)],
        // Bottom-right corner
        (x, y) if x == height && y == width => vec![((y, x), Left), ((y, x), Up)],
        // Top-right corner
        (0, y) if y == height => vec![((y, 0), Left), ((y, 0), Down)],
        // Bottom-left corner
        (x, 0) if x == width => vec![((0, x), Up), ((0, x), Right)],
        // First row
        (x, 0) => vec![((0, x), Down)],
        // First column
        (0, y) => vec![((y, 0), Right)],
        // Last column
        (x, y) if x == width => vec![((y, x), Left)],
        // Last row
        (x, y) if y == width => vec![((y, x), Up)],
        _ => vec![],
      }
    })
    .map(|((y, x), direction)| ((y as isize, x as isize), direction))
    .collect::<Vec<_>>()
}

pub fn part1(input: &str, _opts: Options) -> i64 {
  let grid = parse_grid(input);
  let moveset = ((0isize, 0isize), Direction::Right);

  count_energized(walk(&grid, HashSet::default(), moveset))
}

pub fn part2(input: &str, _opts: Options) -> i64 {
  let grid = parse_grid(input);

  border_start_moves(&grid)
    .into_par_iter()
    .map(|moveset| count_energized(walk(&grid, HashSet::default(), moveset)))
    .max()
    .unwrap()
}
