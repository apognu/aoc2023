use std::collections::{HashSet, VecDeque};

use crate::util;

const ADJACENCY_MATRIX: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn map_gardens(input: &str, max: usize, at: (isize, isize)) -> i64 {
  let grid = util::read_file_lines(input).iter().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
  let mut gardens: HashSet<(isize, isize)> = HashSet::default();
  let mut seen: HashSet<((isize, isize), usize)> = HashSet::default();

  let mut queue: VecDeque<((isize, isize), usize)> = VecDeque::default();
  queue.push_back((at, 0));

  while let Some(iter @ (garden @ (y, x), steps)) = queue.pop_front() {
    if seen.contains(&iter) {
      continue;
    }

    seen.insert(iter);

    if steps == max {
      gardens.insert(garden);
    } else {
      for (y_add, x_add) in ADJACENCY_MATRIX {
        let (y_next, x_next) = (y + y_add, x + x_add);

        if let Some(symbol) = grid
          .get((y_next.rem_euclid(grid.len() as isize)) as usize)
          .and_then(|row| row.get((x_next.rem_euclid(grid[0].len() as isize)) as usize))
        {
          if symbol != &'#' {
            queue.push_back(((y_next, x_next), steps + 1));
          }
        }
      }
    }
  }

  gardens.len() as i64
}

pub fn part1(input: &str) -> i64 {
  map_gardens(input, 64, (65, 65))
}

pub fn part2(_input: &str) -> i64 {
  unimplemented!();
}
