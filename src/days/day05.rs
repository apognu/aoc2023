use std::{cmp, collections::HashMap, ops::Range};

use itertools::Itertools;

use crate::util::{self, parse};

crate::tests!(5, (35, 46));

type Seeds = Vec<i64>;
type ConversionBook = HashMap<String, Vec<ConversionOp>>;
type ConversionOp = ((i64, i64), i64);

fn parse_alamanac(input: &str) -> (Seeds, ConversionBook) {
  let almanac = util::read_file_lines(input);
  let mut seeds: Seeds = vec![];
  let mut mappings: ConversionBook = HashMap::new();

  let mut mapping: String = String::new();

  for line in almanac {
    if line.is_empty() {
      continue;
    }

    if let Some(seeds_list) = line.strip_prefix("seeds: ") {
      seeds = seeds_list.split_whitespace().map(parse::<i64>).collect();

      continue;
    }

    if let Some(conversion) = line.strip_suffix(" map:") {
      mappings.insert(conversion.to_string(), vec![]);
      mapping = conversion.to_string();

      continue;
    }

    let (dest_start, src_start, length) = line.split_whitespace().map(parse::<i64>).collect_tuple().unwrap();
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
    .map(|value| value.iter().collect_tuple().unwrap())
    .map(|(start, length)| (*start..(*start + *length)))
    .collect::<Vec<_>>();

  let all_steps_mins: Vec<_> = CONVERSION_STEPS
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
    .collect();

  find_location_from_seeds(&book, all_steps_mins)
}
