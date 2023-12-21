use std::collections::HashMap;

use crate::util::{self, parse, Options};

crate::tests!(2, (8, 2286));

type Draw = (i64, Vec<(i64, Color)>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Color {
  Red,
  Green,
  Blue,
}

impl From<&str> for Color {
  fn from(value: &str) -> Self {
    use Color::*;

    match value {
      "red" => Red,
      "green" => Green,
      "blue" => Blue,
      _ => panic!("unexpected color"),
    }
  }
}

impl Color {
  const fn max(&self) -> i64 {
    use Color::*;

    match self {
      Red => 12,
      Green => 13,
      Blue => 14,
    }
  }
}

fn parse_game_draws(input: &str) -> Vec<Draw> {
  let lines = util::read_file_lines(input);

  lines
    .into_iter()
    .map(|game| {
      let (game_id, draws) = game.split_once(": ").unwrap();
      let game_id = parse::<i64>(game_id.trim_start_matches("Game "));

      let draws = draws
        .split("; ")
        .flat_map(|draw| draw.split(", ").map(|token| token.split_once(' ').unwrap()).map(|(count, color)| (parse::<i64>(count), color.into())))
        .collect::<Vec<_>>();

      (game_id, draws)
    })
    .collect()
}

type Score = HashMap<Color, i64>;

fn overflow_maxes(colors: &Score) -> bool {
  [Color::Red, Color::Green, Color::Blue].iter().any(|color| colors.get(color).unwrap_or(&0) > &color.max())
}

pub fn part1(input: &str, _opts: Options) -> i64 {
  let draws = parse_game_draws(input);

  let score = draws.iter().fold(0, |acc, (game_id, draws)| {
    let colors = draws.iter().fold(Score::new(), |mut acc, (count, color)| {
      let current_count = acc.get(color).unwrap_or(&0);

      if count > current_count {
        acc.insert(*color, *count);
      }

      acc
    });

    if overflow_maxes(&colors) {
      acc
    } else {
      acc + *game_id
    }
  });

  score
}

pub fn part2(input: &str, _opts: Options) -> i64 {
  let draws = parse_game_draws(input);

  let score = draws.iter().fold(Vec::<i64>::new(), |mut acc, (_, draws)| {
    let colors = draws.iter().fold(Score::new(), |mut acc, (count, color)| {
      let current_count = acc.get(color).unwrap_or(&0);

      if count > current_count {
        acc.insert(*color, *count);
      }

      acc
    });

    acc.push(colors.values().product());
    acc
  });

  score.into_iter().sum()
}
