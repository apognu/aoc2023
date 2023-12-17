use std::iter::repeat;

use itertools::intersperse;

use crate::util::{self, parse};

crate::tests!(12, (21, 525152));

const CACHE_SIZE: usize = 1 << 12;

type Cache<'a> = [i64; CACHE_SIZE];

fn parse_springs(input: &str, folds: usize) -> Vec<(Vec<char>, Vec<u8>)> {
  let lines = util::read_file_lines(input);

  lines
    .iter()
    .flat_map(|line| {
      line.split_once(' ').map(|(springs, counts)| {
        (
          intersperse(repeat(springs).take(folds), "?").flat_map(str::chars).collect::<Vec<_>>(),
          repeat(counts).take(folds).flat_map(|x| x.split(',').map(parse::<u8>)).collect::<Vec<_>>(),
        )
      })
    })
    .collect::<Vec<_>>()
}

fn cache_key(row: &[char], matches: &[u8]) -> usize {
  (row.len() << 5) | matches.len()
}

fn arrangements<'a>(cache: &mut Cache<'a>, row: &'a [char], matches: &'a [u8]) -> i64 {
  match cache[cache_key(row, matches)] {
    i64::MAX => {}
    memoized => return memoized,
  }

  if let ['.', rest @ ..] = row {
    return arrangements(cache, rest, matches);
  }

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

  if symbol == '?' {
    result += arrangements(cache, &row[1..], matches);
  }

  if ['#', '?'].contains(&symbol) && needle <= row.len() && !row[..needle].contains(&'.') && (needle == row.len() || row[needle] != '#') {
    result += arrangements(cache, row.get(needle + 1..).unwrap_or(&[]), &matches[1..])
  }

  cache[cache_key(row, matches)] = result;

  result
}

pub fn part1(input: &str) -> i64 {
  let map = parse_springs(input, 1);
  let mut cache: Cache = [i64::MAX; CACHE_SIZE];

  map
    .iter()
    .map(|(row, broken)| {
      cache.fill(i64::MAX);

      arrangements(&mut cache, row, broken)
    })
    .sum()
}

pub fn part2(input: &str) -> i64 {
  let map = parse_springs(input, 5);
  let mut cache: Cache = [i64::MAX; CACHE_SIZE];

  map
    .iter()
    .map(|(row, broken)| {
      cache.fill(i64::MAX);

      arrangements(&mut cache, row, broken)
    })
    .sum()
}
