use std::collections::{HashMap, HashSet};

use crate::util::{self, parse, Options};

crate::tests!(4, (13, 30));

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

      (parse::<u64>(id.last().unwrap()), line.split_once(" | ").unwrap())
    })
    .map(|(id, (winners, numbers))| Card {
      id,
      numbers: numbers.split_whitespace().map(parse::<i64>).collect(),
      winners: winners.split_whitespace().map(parse::<i64>).collect(),
    })
    .collect();

  cards
}

fn get_card_score(card: &Card) -> u64 {
  card.numbers.intersection(&card.winners).count() as u64
}

pub fn part1(input: &str, _opts: Options) -> i64 {
  let cards = parse_scratch_cards(input);

  cards
    .iter()
    .map(get_card_score)
    .map(|score| {
      (0..score).fold(0, |acc, _| match acc {
        0 => 1,
        n => n * 2,
      })
    })
    .sum()
}

type CardCache = HashMap<u64, i64>;

fn scratch_cards(cards: &Vec<Card>, cache: &mut CardCache, from: usize, count: usize) -> i64 {
  if count == 0 {
    return 0;
  }

  cards[from..(from + count)]
    .iter()
    .enumerate()
    .map(|(index, card)| {
      let score = get_card_score(card);

      let nested_score = match cache.get(&card.id) {
        Some(cached) => *cached,

        None => {
          let score = scratch_cards(cards, cache, from + index + 1, score as usize);
          cache.insert(card.id, score);

          score
        }
      };

      1 + nested_score
    })
    .sum()
}

pub fn part2(input: &str, _opts: Options) -> i64 {
  let cards = parse_scratch_cards(input);
  let mut cache: CardCache = HashMap::new();

  scratch_cards(&cards, &mut cache, 0, cards.len())
}
