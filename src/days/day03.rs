use std::{cmp, collections::HashMap};

use crate::util::{self, parse};

type Grid = Vec<Vec<char>>;

#[derive(Debug)]
struct EnginePart {
  number: i64,
  row: usize,
  col: usize,
  length: usize,
}

fn get_coords_of_parts(input: &str) -> (Grid, Vec<EnginePart>) {
  let lines: Vec<Vec<char>> = util::read_file_lines(input).into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect();

  let (_, cols) = (lines.len(), lines.get(0).unwrap().len());
  let mut parts: Vec<EnginePart> = vec![];

  for (row, chars) in lines.iter().enumerate() {
    let mut col = 0;

    while col <= chars.len() {
      let mut result = String::with_capacity(cols);

      for length in 1..(chars.len() - col + 1) {
        let view = &chars[col..(col + length)];

        if !view.iter().all(|c| c.is_ascii_digit()) {
          break;
        }

        if let Some(part) = view.last() {
          result.push(*part);
        }
      }

      if !result.is_empty() {
        parts.push(EnginePart {
          number: parse::<i64>(&result),
          row,
          col,
          length: result.len(),
        });
      }

      col += cmp::max(result.len(), 1);
    }
  }

  (lines, parts)
}

const ADJACENCY_MATRIX: [(isize, isize); 8] = [(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];

fn get_char_at(grid: &Grid, (row, col): (isize, isize)) -> Option<char> {
  if let Some(symbol) = grid.get(row as usize).and_then(|row| row.get(col as usize)) {
    return Some(*symbol);
  }

  None
}

fn is_part_adjacent(grid: &Grid, part: &EnginePart) -> bool {
  for x in 0..part.length {
    for (y_neigh, x_neigh) in ADJACENCY_MATRIX {
      let row = part.row as isize + y_neigh;
      let col = part.col as isize + x as isize + x_neigh;

      if let Some(symbol) = get_char_at(grid, (row, col)) {
        if !symbol.is_ascii_digit() && symbol != '.' {
          return true;
        }
      }
    }
  }

  false
}

fn find_adjacent_gears(grid: &Grid, part: &EnginePart) -> (Vec<(isize, isize)>, i64) {
  let mut gears: Vec<(isize, isize)> = vec![];

  for x in 0..part.length {
    for (y_neigh, x_neigh) in ADJACENCY_MATRIX {
      let row = part.row as isize + y_neigh;
      let col = part.col as isize + x as isize + x_neigh;

      if let Some(symbol) = get_char_at(grid, (row, col)) {
        if symbol == '*' {
          let coords = (row, col);

          if !gears.contains(&coords) {
            gears.push((row, col));
          }
        }
      }
    }
  }

  (gears, part.number)
}

pub fn part1(input: &str) -> i64 {
  let (grid, parts) = get_coords_of_parts(input);

  parts.iter().filter(|part| is_part_adjacent(&grid, part)).map(|part| part.number).sum()
}

pub fn part2(input: &str) -> i64 {
  let (grid, parts) = get_coords_of_parts(input);

  let gears: Vec<(Vec<(isize, isize)>, i64)> = parts.iter().map(|part| find_adjacent_gears(&grid, part)).collect();

  let mut gear_parts: HashMap<(isize, isize), Vec<i64>> = HashMap::new();

  for (gears_coords, part) in gears {
    for gear in gears_coords {
      gear_parts.entry(gear).or_default().push(part);
    }
  }

  gear_parts.into_iter().filter(|(_, parts)| parts.len() == 2).map(|(_, parts)| parts.iter().product::<i64>()).sum()
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    assert_eq!(super::part1("03_test.txt"), 4361);
  }

  #[test]
  fn part2() {
    assert_eq!(super::part2("03_test.txt"), 467835);
  }
}
