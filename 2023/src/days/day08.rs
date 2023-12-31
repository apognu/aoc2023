use std::collections::HashMap;

use itertools::{FoldWhile::*, Itertools};
use num::integer::lcm;

use crate::util::{self, Options};

crate::tests!(8, (6, 6));

type Turns = Vec<char>;
type Map = HashMap<String, (String, String)>;

fn parse_steps(input: &str) -> (Turns, Map) {
  let lines = util::read_file_lines(input);
  let mut lines = lines.iter();

  let turns = lines.next().unwrap();
  let turns = turns.chars().collect::<Vec<_>>();

  let map = lines
    .filter_map(|step| match step.is_empty() {
      true => None,
      false => {
        let (at, directions) = step.split_once(" = ").unwrap();
        let (left, right) = directions.split_once(", ").unwrap();

        Some((at.to_string(), (left.trim_start_matches('(').to_string(), right.trim_end_matches(')').to_string())))
      }
    })
    .collect::<HashMap<_, _>>();

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

pub fn part1(input: &str, _opts: Options) -> i64 {
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

pub fn part2(input: &str, _opts: Options) -> i64 {
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
