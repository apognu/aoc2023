use std::collections::VecDeque;

use itertools::Itertools;

use crate::util::{self, parse, Options};

crate::tests!(18, (62, 952408144115));

#[derive(Debug, Clone, Copy)]
enum Direction {
  Up,
  Left,
  Right,
  Down,
}

impl From<&str> for Direction {
  fn from(value: &str) -> Self {
    use Direction::*;

    match value {
      "R" | "0" => Right,
      "D" | "1" => Down,
      "L" | "2" => Left,
      "U" | "3" => Up,
      _ => panic!("unexpected character: {value}"),
    }
  }
}

type Coord = (isize, isize);

struct Hex<'a>(&'a str);

#[derive(Debug, Clone, Copy)]
struct Move {
  direction: Direction,
  distance: usize,
}

impl From<&str> for Move {
  fn from(value: &str) -> Self {
    let (direction, distance) = value.split_ascii_whitespace().next_tuple().unwrap();

    Move {
      direction: Direction::from(direction),
      distance: parse::<usize>(distance),
    }
  }
}

impl<'a> From<Hex<'a>> for Move {
  fn from(value: Hex<'a>) -> Self {
    let hexstring = value.0.trim_end_matches(')');
    let distance = &hexstring[0..5];
    let direction = &hexstring[5..];

    Move {
      direction: Direction::from(direction),
      distance: usize::from_str_radix(distance, 16).unwrap(),
    }
  }
}

fn parse_trench(input: &str, with_hex: bool) -> VecDeque<Move> {
  util::read_file_lines(input)
    .iter()
    .map(|line| {
      let (int, hex) = line.split("(#").next_tuple().unwrap();
      match with_hex {
        false => Move::from(int),
        true => Move::from(Hex(hex)),
      }
    })
    .collect::<VecDeque<_>>()
}

fn dig(mut trenches: VecDeque<Move>) -> Vec<Coord> {
  use Direction::*;

  let mut coord = (0, 0);
  let mut vertices = vec![(0, 0)];

  while let Some(Move { direction, distance }) = trenches.pop_front() {
    if trenches.is_empty() {
      vertices.push((0, 0));
      break;
    }

    let (x, y) = coord;
    let distance = distance as isize;

    coord = match direction {
      Up => (x, y - distance),
      Left => (x - distance, y),
      Right => (x + distance, y),
      Down => (x, y + distance),
    };

    vertices.push(coord);
  }

  vertices
}

// Shoelace formula
fn lace_shoe((x1, y1): Coord, (x2, y2): Coord) -> isize {
  (x1 * y2) - (y1 * x2)
}

fn perimeter(trenches: &VecDeque<Move>) -> isize {
  trenches.iter().fold(0, |distance, movement| distance + movement.distance as isize)
}

fn area(vertices: &[Coord], distance: isize) -> i64 {
  let mut area = 0;

  vertices.windows(2).for_each(|two| {
    let left = two[0];
    let right = two[1];

    area += lace_shoe(left, right);
  });

  // Pick's theorem
  let area = (area / 2).abs() - distance / 2 + 1;

  (distance + area) as i64
}

pub fn part1(input: &str, _opts: Options) -> i64 {
  let trenches = parse_trench(input, false);
  let distance = perimeter(&trenches);
  let vertices = dig(trenches);

  area(&vertices, distance)
}

pub fn part2(input: &str, _opts: Options) -> i64 {
  let trenches = parse_trench(input, true);
  let distance = perimeter(&trenches);
  let vertices = dig(trenches);

  area(&vertices, distance)
}
