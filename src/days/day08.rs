use std::collections::HashMap;

use itertools::{FoldWhile::*, Itertools};
use num::integer::lcm;

use crate::util;

type Turns = Vec<char>;
type Map = HashMap<String, (String, String)>;

fn parse_steps(input: &str) -> (Turns, Map) {
  let lines = util::read_file_lines(input);
  let mut lines = lines.iter();

  let turns = lines.next().unwrap();
  let turns = turns.chars().collect::<Vec<_>>();

  let mut map: Map = HashMap::new();

  for step in lines {
    if step.is_empty() {
      continue;
    }

    let (at, directions) = step.split_once(" = ").unwrap();
    let (left, right) = directions.split_once(", ").unwrap();

    map.insert(at.to_string(), (left.trim_start_matches('(').to_string(), right.trim_end_matches(')').to_string()));
  }

  (turns, map)
}

fn next_step(map: &Map, at: &str, to: &char) -> String {
  let next = map.get(at).unwrap();

  let next = match to {
    'L' => &next.0,
    'R' => &next.1,
    _ => panic!("not a valid direction"),
  };

  next.to_string()
}

pub fn part1(input: &str) -> i64 {
  let (turns, map) = parse_steps(input);

  let (_, index) = turns
    .iter()
    .cycle()
    .fold_while(("AAA".to_string(), 0), |(at, index), to| match at.as_str() {
      "ZZZ" => Done((at, index)),
      _ => Continue((next_step(&map, &at, to), index + 1)),
    })
    .into_inner();

  index
}

pub fn part2(input: &str) -> i64 {
  let (turns, map) = parse_steps(input);

  map
    .iter()
    .filter_map(|(start, _)| match start.ends_with('A') {
      false => None,
      true => {
        let index = turns
          .iter()
          .cycle()
          .fold_while((start.to_string(), 0), |(at, index), to| match at.ends_with('Z') {
            true => Done((at, index)),
            false => Continue((next_step(&map, &at, to), index + 1)),
          })
          .into_inner();

        Some(index)
      }
    })
    .map(|(_, index)| index as i64)
    .fold(1, lcm)
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    assert_eq!(super::part1("08_1_test.txt"), 6);
  }

  #[test]
  fn part2() {
    assert_eq!(super::part2("08_2_test.txt"), 6);
  }
}
