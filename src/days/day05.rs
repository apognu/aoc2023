use std::{cmp, collections::HashMap, ops::Range};

use crate::util;

type Seeds = Vec<i64>;
type ConversionBook = HashMap<String, Vec<ConversionOp>>;
type ConversionOp = ((i64, i64), i64);

fn parse_alamanac(input: &str) -> (Seeds, ConversionBook) {
  let almanac = util::read_file_lines(input);
  let mut seeds: Seeds = vec![];
  let mut mappings: ConversionBook = HashMap::new();
  let mut current_mapping: String = String::new();

  for line in almanac {
    if line.is_empty() {
      continue;
    }

    let mapping = current_mapping.clone();

    if let Some(seeds_list) = line.strip_prefix("seeds: ") {
      seeds = seeds_list.split_whitespace().map(|seed| seed.parse::<i64>().unwrap()).collect();

      continue;
    }

    if let Some(conversion) = line.strip_suffix(" map:") {
      mappings.insert(conversion.to_string(), vec![]);
      current_mapping = conversion.to_string();

      continue;
    }

    let mut tokens = line.split_whitespace().map(|token| token.parse::<i64>().unwrap());
    let (dest_start, src_start, length) = (tokens.next().unwrap(), tokens.next().unwrap(), tokens.next().unwrap());

    let entry = ((src_start, src_start + length), dest_start - src_start);

    mappings.get_mut(mapping.as_str()).unwrap().push(entry);
  }

  (seeds, mappings)
}

const CONVERSION_STEPS: [&str; 7] = [
  "seed-to-soil",
  "soil-to-fertilizer",
  "fertilizer-to-water",
  "water-to-light",
  "light-to-temperature",
  "temperature-to-humidity",
  "humidity-to-location",
];

fn find_location_from_seeds(book: &ConversionBook, seeds: Vec<i64>) -> i64 {
  seeds.into_iter().fold(std::i64::MAX, |min_location, seed| {
    let location = CONVERSION_STEPS.into_iter().fold(seed, |value, step| {
      let ops = book.get(step).unwrap();

      match ops.iter().find(|((start, end), _)| (start..end).contains(&&value)) {
        None => value,
        Some((_, op)) => value + op,
      }
    });

    cmp::min(location, min_location)
  })
}

pub fn part1(input: &str) -> i64 {
  let (seeds, book) = parse_alamanac(input);

  find_location_from_seeds(&book, seeds)
}

fn find_seed_from_location_and_step(book: &ConversionBook, upto: usize, seeds: &[Range<i64>], location: i64) -> Option<i64> {
  let mut value = location;

  'step: for step in CONVERSION_STEPS[0..upto].iter().rev() {
    for ((start, end), op) in book.get(*step).unwrap().clone() {
      if ((start + op)..(end + op)).contains(&value) {
        value -= op;
        continue 'step;
      }
    }
  }

  if seeds.iter().any(|range| range.contains(&(value))) {
    Some(value)
  } else {
    None
  }
}

pub fn part2(input: &str) -> i64 {
  let (seeds, book) = parse_alamanac(input);

  let seeds = seeds
    .chunks_exact(2)
    .map(|value| (*value.first().unwrap(), (*value.last().unwrap())))
    .map(|(start, length)| (start..(start + length)))
    .collect::<Vec<_>>();

  let bounds = CONVERSION_STEPS
    .into_iter()
    .enumerate()
    .flat_map(|(index, step)| {
      book
        .get(step)
        .unwrap()
        .iter()
        .map(|((low, _), _p)| *low)
        .filter_map(|loc| find_seed_from_location_and_step(&book, index, &seeds, loc))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  find_location_from_seeds(&book, bounds)
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    assert_eq!(super::part1("05_test.txt"), 35);
  }

  #[test]
  fn part2() {
    assert_eq!(super::part2("05_test.txt"), 46);
  }
}
