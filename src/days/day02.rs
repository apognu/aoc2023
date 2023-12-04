use std::collections::HashMap;

use crate::util;

fn parse_game_draws(input: &str) -> Vec<(i64, Vec<(i64, String)>)> {
  let lines = util::read_file_lines(input);

  let draws: Vec<(i64, Vec<(i64, String)>)> = lines
    .into_iter()
    .map(|game| {
      let (game_id, draws) = game.split_once(": ").unwrap();
      let game_id = game_id.trim_start_matches("Game ").parse::<i64>().unwrap();

      let draws = draws
        .split("; ")
        .flat_map(|draw| {
          draw
            .split(", ")
            .map(|token| token.split_once(' ').unwrap())
            .map(|(count, color)| (count.parse::<i64>().unwrap(), color.to_string()))
        })
        .collect::<Vec<(i64, String)>>();

      (game_id, draws)
    })
    .collect();

  draws
}

const MAX_RED: i64 = 12;
const MAX_GREEN: i64 = 13;
const MAX_BLUE: i64 = 14;

pub fn part1(input: &str) -> i64 {
  let draws = parse_game_draws(input);

  let score = draws.iter().fold(0, |acc, (game_id, draws)| {
    let colors = draws.iter().fold(HashMap::<String, i64>::new(), |mut acc, (count, color)| {
      let current_count = acc.get(color.as_str()).unwrap_or(&0);

      if count > current_count {
        acc.insert(color.to_string(), *count);
      }

      acc
    });

    if colors.get("red").unwrap_or(&0) > &MAX_RED || colors.get("green").unwrap_or(&0) > &MAX_GREEN || colors.get("blue").unwrap_or(&0) > &MAX_BLUE {
      acc
    } else {
      acc + *game_id
    }
  });

  score
}

pub fn part2(input: &str) -> i64 {
  let draws = parse_game_draws(input);

  let score = draws.iter().fold(Vec::<i64>::new(), |mut acc, (_, draws)| {
    let colors = draws.iter().fold(HashMap::<String, i64>::new(), |mut acc, (count, color)| {
      let current_count = acc.get(color.as_str()).unwrap_or(&0);

      if count > current_count {
        acc.insert(color.to_string(), *count);
      }

      acc
    });

    acc.push(colors.values().product());
    acc
  });

  score.into_iter().sum()
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    assert_eq!(super::part1("02_test.txt"), 8);
  }

  #[test]
  fn part2() {
    assert_eq!(super::part2("02_test.txt"), 2286);
  }
}
