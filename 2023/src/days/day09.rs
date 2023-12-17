use itertools::Itertools;

use crate::util::{self, parse};

crate::tests!(9, (114, 2));

fn parse_predictions(input: &str) -> Vec<Vec<i64>> {
  let input = util::read_file_lines(input);
  let input = input.into_iter().map(|x| x.split_ascii_whitespace().map(parse::<i64>).collect::<Vec<_>>()).collect::<Vec<_>>();

  input
}

fn compute_next_value(predictions: Vec<Vec<i64>>) -> i64 {
  let sum = predictions.iter().fold(0, |acc, series| {
    let mut value = series.clone();
    let mut adds = *value.last().unwrap();

    while !value.iter().all_equal() {
      let new = value.iter().tuple_windows().fold(vec![], |mut acc, (left, right)| {
        acc.push(right - left);
        acc
      });

      adds += new.last().unwrap();
      value = new;
    }

    acc + adds
  });

  sum
}

pub fn part1(input: &str) -> i64 {
  let predictions = parse_predictions(input);

  compute_next_value(predictions)
}

pub fn part2(input: &str) -> i64 {
  let predictions = parse_predictions(input)
    .into_iter()
    .map(|mut series| {
      series.reverse();
      series
    })
    .collect::<Vec<_>>();

  compute_next_value(predictions)
}
