use crate::util::{self, parse, Options};

crate::tests!(1, (142, 281));

fn first_and_last_digit<S>(string: S) -> i64
where
  S: AsRef<str>,
{
  let chars = string.as_ref().chars();

  let first = chars.clone().find(|char| char.is_ascii_digit()).unwrap();
  let last = chars.rev().find(|char| char.is_ascii_digit()).unwrap();

  parse::<i64>(&format!("{}{}", first, last))
}

static LETTERS: &[(&str, &str)] = &[
  ("one", "o1e"),
  ("two", "t2o"),
  ("six", "s6x"),
  ("four", "f4r"),
  ("five", "f5e"),
  ("nine", "n9e"),
  ("three", "t3e"),
  ("seven", "s7n"),
  ("eight", "e8t"),
];

fn replace_words_with_digits<S>(string: S) -> String
where
  S: AsRef<str>,
{
  LETTERS.iter().fold(string.as_ref().to_string(), |acc, (word, value)| acc.replace(word, value))
}

pub fn part1(input: &str, _opts: Options) -> i64 {
  let lines = util::read_file_lines(input);

  lines.iter().map(first_and_last_digit).sum()
}

pub fn part2(input: &str, _opts: Options) -> i64 {
  let lines = util::read_file_lines(input);

  lines.iter().map(replace_words_with_digits).map(first_and_last_digit).sum()
}
