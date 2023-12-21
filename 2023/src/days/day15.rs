use std::collections::HashMap;

use crate::util::{self, parse, Options};

crate::tests!(15, (1320, 145));

fn parse_sequence(input: &str) -> Vec<String> {
  util::read_file_lines(input).join("").split(',').map(String::from).collect::<Vec<_>>()
}

fn hash<S>(item: S) -> i64
where
  S: AsRef<str>,
{
  item.as_ref().chars().fold(0, |acc, item| {
    let code = item as i64;

    ((acc + code) * 17) % 256
  })
}

fn verification_number(items: &[String]) -> i64 {
  items.iter().map(hash).sum()
}

enum Op {
  Set,
  Remove,
}

const fn box_refraction_factor(box_id: i64, index: usize, power: i64) -> i64 {
  (box_id + 1) * (index as i64 + 1) * power
}

fn little_boxes_on_the_hillside(items: &[String]) -> i64 {
  let mut boxes: HashMap<i64, Vec<(String, i64)>> = HashMap::new();

  for item in items {
    let (op, label, value) = if item.contains('=') {
      let (label, value) = item.split_once('=').unwrap();

      (Op::Set, label, parse::<i64>(value))
    } else {
      let (label, _) = item.split_once('-').unwrap();

      (Op::Remove, label, 0)
    };

    let box_id = hash(label);

    match op {
      Op::Set => {
        let slot = boxes.entry(box_id).or_default();

        if let Some(index) = slot.iter().position(|(l, _)| label == l) {
          slot[index] = (label.to_string(), value);
        } else {
          slot.push((label.to_string(), value));
        }
      }

      Op::Remove => {
        if let Some(slot) = boxes.get_mut(&box_id) {
          if let Some(index) = slot.iter().position(|(l, _)| label == l) {
            slot.remove(index);
          }
        };
      }
    }
  }

  boxes
    .into_iter()
    .flat_map(|(box_id, steps)| steps.into_iter().enumerate().map(move |(index, (_, power))| box_refraction_factor(box_id, index, power)))
    .sum()
}

pub fn part1(input: &str, _opts: Options) -> i64 {
  verification_number(&parse_sequence(input))
}

pub fn part2(input: &str, _opts: Options) -> i64 {
  little_boxes_on_the_hillside(&parse_sequence(input))
}
