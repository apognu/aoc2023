use std::{
  collections::{HashMap, HashSet},
  ops::Neg,
};

use crate::util::{self, Options};

type Maze = HashMap<Coord, Directions>;
type Coord = (usize, usize);

crate::tests!(10, (8, 10));

fn parse_maze(input: &str) -> (Maze, Coord) {
  let input = util::read_file_lines(input);
  let mut maze: Maze = HashMap::new();
  let mut start: Option<Coord> = None;

  for (row, cols) in input.iter().enumerate() {
    for (col, symbol) in cols.chars().enumerate() {
      if symbol == '.' {
        continue;
      }
      if symbol == 'S' {
        start = Some((col, row));
      }

      maze.insert((col, row), symbol.into());
    }
  }

  (maze, start.unwrap())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
  North,
  South,
  West,
  East,
}

impl Neg for Direction {
  type Output = Self;

  fn neg(self) -> Self::Output {
    use Direction::*;

    match self {
      North => South,
      South => North,
      West => East,
      East => West,
    }
  }
}

impl Direction {
  fn go(&self, (col, row): Coord) -> (isize, isize) {
    use Direction::*;

    let (col, row) = (col as isize, row as isize);

    match self {
      North => (col, row - 1),
      South => (col, row + 1),
      West => (col - 1, row),
      East => (col + 1, row),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Directions(HashSet<Direction>);

impl From<char> for Directions {
  fn from(value: char) -> Self {
    use Direction::*;

    match value {
      'S' => Directions(HashSet::from_iter([North, West, East, South])),
      '|' => Directions(HashSet::from_iter([North, South])),
      '-' => Directions(HashSet::from_iter([West, East])),
      'L' => Directions(HashSet::from_iter([North, East])),
      '7' => Directions(HashSet::from_iter([West, South])),
      'J' => Directions(HashSet::from_iter([North, West])),
      'F' => Directions(HashSet::from_iter([East, South])),
      _ => Directions(HashSet::new()),
    }
  }
}

fn first_tile(maze: &Maze, start: Coord) -> (Direction, Coord) {
  let (direction, col, row) = [Direction::North, Direction::South, Direction::West, Direction::East]
    .into_iter()
    .filter_map(|direction| {
      let (col, row) = direction.go(start);

      if col < 0 || row < 0 {
        return None;
      }

      maze
        .get(&(col as usize, row as usize))
        .and_then(|candidate| match candidate.0.contains(&-direction) {
          true => Some(candidate),
          false => None,
        })
        .map(|_| (direction, col, row))
    })
    .nth(1)
    .unwrap();

  (direction, (col as usize, row as usize))
}

// Shoelace formula
fn lace_shoe((x1, y1): Coord, (x2, y2): Coord) -> isize {
  (x1 as isize * y2 as isize) - (y1 as isize * x2 as isize)
}

fn walk_the_maze(maze: &Maze, start: &Coord, mut at: Coord, mut previous: Direction) -> (i64, i64) {
  let mut distance = 1;
  let mut area = lace_shoe(*start, at);

  while start != &at {
    let directions = maze.get(&at).unwrap();

    for direction in &directions.0 {
      if *direction == -previous {
        continue;
      }

      let (col, row) = direction.go(at);
      let next = (col as usize, row as usize);

      area += lace_shoe((at.0, at.1), next);

      distance += 1;
      at = next;
      previous = *direction;

      break;
    }
  }

  // Pick's theorem
  (distance / 2, (area as i64 / 2).abs() - distance / 2 + 1)
}

pub fn part1(input: &str, _opts: Options) -> i64 {
  let (maze, start) = parse_maze(input);
  let (direction, at) = first_tile(&maze, start);
  let (distance, _) = walk_the_maze(&maze, &start, at, direction);

  distance
}

pub fn part2(input: &str, _opts: Options) -> i64 {
  let (maze, start) = parse_maze(input);
  let (direction, at) = first_tile(&maze, start);
  let (_, area) = walk_the_maze(&maze, &start, at, direction);

  area
}
