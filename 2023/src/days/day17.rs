use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap},
};

use crate::util::{self, parse};

crate::tests!(17, (102, 94));

type Grid = Vec<Vec<i64>>;
type Coord = (isize, isize);
type Queue = BinaryHeap<Reverse<State>>;
type Cache = HashMap<(Direction, Coord), i64>;
type MinMax = (usize, usize);

#[derive(Debug, Eq)]
struct State {
  cost: i64,
  direction: Direction,
  coord: Coord,
}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.cost == other.cost
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.cost.cmp(&other.cost)
  }
}

fn parse_grid(input: &str) -> Grid {
  util::read_file_lines(input)
    .iter()
    .map(|row| row.chars().map(|c| parse::<i64>(&c.to_string())).collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Left,
  Right,
  Down,
}

fn next_for_direction((y, x): Coord, direction: Direction) -> Coord {
  use Direction::*;

  match direction {
    Up => (y - 1, x),
    Left => (y, x - 1),
    Right => (y, x + 1),
    Down => (y + 1, x),
  }
}

fn slide_crucible(grid: &Grid, queue: &mut Queue, seen: &mut Cache, mut at: Coord, mut cost: i64, (min, max): MinMax, direction: Direction) {
  for step in 1..=max {
    at = next_for_direction(at, direction);

    let (y, x) = at;

    if let Some(step_cost) = grid.get(y as usize).and_then(|r| r.get(x as usize)) {
      cost += step_cost;

      // If we already passed through this tile in the same direction and had a
      // lower cost, the current path is going to be longer (the rest will
      // follow the same path).
      if step >= min && cost < *seen.get(&(direction, at)).unwrap_or(&i64::MAX) {
        queue.push(Reverse(State { cost, direction, coord: at }));
        seen.insert((direction, at), cost);
      }
    }
  }
}

fn shortest(grid: &Grid, (min, max): MinMax) -> i64 {
  use Direction::*;

  let end = (grid.len() as isize - 1, grid[0].len() as isize - 1);

  let mut queue = BinaryHeap::new();
  let mut seen: Cache = HashMap::default();

  for init in [Down, Right] {
    queue.push(Reverse(State {
      cost: 0,
      direction: init,
      coord: (0, 0),
    }));
  }

  while let Some(Reverse(State { cost, direction, coord: at })) = queue.pop() {
    if at == end {
      return cost;
    }

    let turns = match direction {
      Up | Down => [Left, Right],
      Left | Right => [Up, Down],
    };

    for turn in turns {
      slide_crucible(grid, &mut queue, &mut seen, at, cost, (min, max), turn);
    }
  }

  panic!("should not be reached");
}

pub fn part1(input: &str) -> i64 {
  let grid = parse_grid(input);

  shortest(&grid, (0, 3))
}

pub fn part2(input: &str) -> i64 {
  let grid = parse_grid(input);

  shortest(&grid, (4, 10))
}
