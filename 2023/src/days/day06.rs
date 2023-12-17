use itertools::Itertools;

use crate::util::{self, parse};

crate::tests!(6, (288, 71503));

fn parse_races(input: &str, single_race: bool) -> Vec<(i64, i64)> {
  let mut races = util::read_file_lines(input).into_iter();

  let times = races.next().unwrap();
  let distances = races.next().unwrap();

  if single_race {
    let time = parse::<i64>(&times.trim_start_matches("Time:").split_whitespace().join(""));
    let distance = parse::<i64>(&distances.trim_start_matches("Distance:").split_whitespace().join(""));

    vec![(time, distance)]
  } else {
    let times = times.trim_start_matches("Time:").split_whitespace().map(parse::<i64>);
    let distances = distances.trim_start_matches("Distance:").split_whitespace().map(parse::<i64>);

    times.zip(distances).collect()
  }
}

fn compute_winning_combinations(races: &[(i64, i64)]) -> i64 {
  races
    .iter()
    .map(|(time, distance)| {
      let time = *time as f64;
      let distance = *distance as f64;

      let high = (time + (time.powi(2) - (4.0 * distance)).sqrt()) / 2.0;
      let low = (time - (time.powi(2) - (4.0 * distance)).sqrt()) / 2.0;

      ((high.floor() - low.ceil()) + (if high.fract() + low.fract() == 0.0 { -1.0 } else { 1.0 })) as i64
    })
    .product()
}

pub fn part1(input: &str) -> i64 {
  let races = parse_races(input, false);

  compute_winning_combinations(&races)
}

pub fn part2(input: &str) -> i64 {
  let races = parse_races(input, true);

  compute_winning_combinations(&races)
}
