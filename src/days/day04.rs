use std::collections::HashSet;

use crate::util;

#[derive(Debug)]
struct Card {
  id: u64,
  numbers: HashSet<i64>,
  winners: HashSet<i64>,
}

fn parse_scratch_cards(input: &str) -> Vec<Card> {
  let cards: Vec<Card> = util::read_file_lines(input)
    .iter()
    .map(|line| line.split_once(": ").unwrap())
    .map(|(id, line)| {
      let id = id.split_whitespace();

      (id.last().unwrap().parse::<u64>().unwrap(), line.split_once(" | ").unwrap())
    })
    .map(|(id, (winners, numbers))| Card {
      id,
      numbers: numbers.split_whitespace().map(|i| i.parse::<i64>().unwrap()).collect(),
      winners: winners.split_whitespace().map(|i| i.parse::<i64>().unwrap()).collect(),
    })
    .collect();

  cards
}

pub fn part1(input: &str) -> i64 {
  let cards = parse_scratch_cards(input);

  cards
    .iter()
    .map(|card| card.numbers.intersection(&card.winners).count())
    .map(|score| {
      (0..score).fold(0, |acc, _| match acc {
        0 => 1,
        n => n * 2,
      })
    })
    .sum()
}

fn scratch_cards(cards: &Vec<Card>, from: usize, count: usize) -> i64 {
  if count == 0 {
    return 0;
  }

  cards[from..(from + count)]
    .iter()
    .enumerate()
    .map(|(index, card)| {
      let score = get_card_score(card);
      let nested_score = scratch_cards(cards, from + index + 1, score as usize);

      1 + nested_score
    })
    .sum()
}

fn get_card_score(card: &Card) -> u64 {
  card.numbers.intersection(&card.winners).count() as u64
}

pub fn part2(input: &str) -> i64 {
  let cards = parse_scratch_cards(input);

  scratch_cards(&cards, 0, cards.len())
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    assert_eq!(super::part1("04_test.txt"), 13);
  }

  #[test]
  fn part2() {
    assert_eq!(super::part2("04_test.txt"), 30);
  }
}
