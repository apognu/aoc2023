use itertools::Itertools;

use crate::util::{self, Options};

crate::tests!(11, (374, 82000210));

type Coord = (usize, usize);

fn compute_universe_expansion(universe: &[Vec<char>], factor: usize) -> (Vec<usize>, Vec<usize>) {
  let mut col_offsets: Vec<usize> = vec![];
  let mut row_offsets: Vec<usize> = vec![];

  for row_id in 0..universe.len() {
    let new = col_offsets.last().unwrap_or(&0);

    match universe.get(row_id).unwrap().iter().all_equal_value() {
      Ok('.') => col_offsets.push(new + factor - 1),
      _ => col_offsets.push(*new),
    }

    let new = row_offsets.last().unwrap_or(&0);
    let mut col = universe.iter().map(|row| row.get(row_id).unwrap());

    match col.all_equal_value() {
      Ok('.') => row_offsets.push(new + factor - 1),
      _ => row_offsets.push(*new),
    }
  }

  (col_offsets, row_offsets)
}

fn map_universe(input: &str, expansion_factor: usize) -> Vec<Coord> {
  let mut universe: Vec<Vec<char>> = vec![];
  let mut galaxies: Vec<Coord> = vec![];

  for row in util::read_file_lines(input).iter() {
    universe.push(row.chars().collect::<Vec<_>>());
  }

  let (col_offsets, row_offsets) = compute_universe_expansion(&universe, expansion_factor);

  for (x, row) in universe.iter().enumerate() {
    for (y, symbol) in row.iter().enumerate() {
      if symbol == &'#' {
        galaxies.push((x + col_offsets.get(x).unwrap(), y + row_offsets.get(y).unwrap()));
      }
    }
  }

  galaxies
}

fn manhattan_distance_of_life_the_universe_and_everything(galaxies: &[Coord]) -> i64 {
  galaxies
    .iter()
    .tuple_combinations()
    .map(|((x1, y1), (x2, y2))| (*x1 as i64 - *x2 as i64).abs() + (*y1 as i64 - *y2 as i64).abs())
    .sum()
}

pub fn part1(input: &str, _opts: Options) -> i64 {
  let galaxies = map_universe(input, 2);

  manhattan_distance_of_life_the_universe_and_everything(&galaxies)
}

pub fn part2(input: &str, _opts: Options) -> i64 {
  let galaxies = map_universe(input, 1_000_000);

  manhattan_distance_of_life_the_universe_and_everything(&galaxies)
}
