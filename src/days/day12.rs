use std::iter::repeat;

use itertools::intersperse;
use rustc_hash::FxHashMap;

use crate::util::{self, parse};

crate::tests!(12, (21, 525152));

type Cache<'a> = FxHashMap<(&'a [char], &'a [i64]), i64>;

fn parse_springs(input: &str, folds: usize) -> Vec<(Vec<char>, Vec<i64>)> {
  let lines = util::read_file_lines(input);

  lines
    .iter()
    .flat_map(|line| {
      line.split_once(' ').map(|(springs, counts)| {
        (
          intersperse(repeat(springs).take(folds), "?").flat_map(str::chars).collect::<Vec<_>>(),
          repeat(counts).take(folds).flat_map(|x| x.split(',').map(parse::<i64>)).collect::<Vec<_>>(),
        )
      })
    })
    .collect::<Vec<_>>()
}

fn arrangements<'a>(cache: &mut Cache<'a>, row: &'a [char], matches: &'a [i64]) -> i64 {
  cache.get(&(row, matches)).copied().unwrap_or_else(|| {
    if row.is_empty() {
      return match matches {
        [] => 1,
        _ => 0,
      };
    }

    if matches.is_empty() {
      return match row.contains(&'#') {
        true => 0,
        false => 1,
      };
    }

    let mut result = 0;

    let needle = matches[0] as usize;
    let symbol = row[0];

    if ['.', '?'].contains(&symbol) {
      result += arrangements(cache, &row[1..], matches);
    }

    if ['#', '?'].contains(&symbol) && needle <= row.len() && !row[..needle].contains(&'.') && (needle == row.len() || row[needle] != '#') {
      result += arrangements(cache, row.get(needle + 1..).unwrap_or(&[]), &matches[1..])
    }

    cache.insert((row, matches), result);

    result
  })
}

pub fn part1(input: &str) -> i64 {
  let map = parse_springs(input, 1);
  let mut cache: Cache = FxHashMap::default();

  map.iter().map(|(row, broken)| arrangements(&mut cache, row, broken)).sum()
}

pub fn part2(input: &str) -> i64 {
  let map = parse_springs(input, 5);
  let mut cache: Cache = FxHashMap::default();

  map.iter().map(|(row, broken)| arrangements(&mut cache, row, broken)).sum()
}
