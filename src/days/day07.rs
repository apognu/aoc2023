use std::cmp::Ordering;

use itertools::Itertools;

use crate::util::{self, parse};

#[derive(Debug, Eq, PartialEq)]
struct Hand(i64, i64, i64);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Suit {
  Value(usize),
  Jack,
  Queen,
  King,
  Ace,
  Joker,
}

impl From<Suit> for usize {
  fn from(suit: Suit) -> Self {
    use Suit::*;

    match suit {
      Value(n) => n,
      Jack => 11,
      Queen => 12,
      King => 13,
      Ace => 14,
      Joker => 1,
    }
  }
}

impl From<char> for Suit {
  fn from(value: char) -> Self {
    use Suit::*;

    match value {
      face @ '2'..='9' => Value(parse::<usize>(face.to_string().as_str())),
      'T' => Value(10),
      'J' => Jack,
      'Q' => Queen,
      'K' => King,
      'A' => Ace,
      _ => panic!("unknown card"),
    }
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Hand {
  fn cmp(&self, Hand(rhand, rraw, _): &Self) -> Ordering {
    let Hand(lhand, lraw, _) = self;

    (lhand, lraw).cmp(&(rhand, rraw))
  }
}

fn hand_value(value: &[usize]) -> i64 {
  let ten: i64 = 10;

  match value {
    &[5] => ten.pow(6),
    &[4] => ten.pow(5),
    &[3, 2] | &[2, 3] => ten.pow(4),
    &[3] => ten.pow(3),
    &[2, 2] => ten.pow(2),
    &[2] => ten.pow(1),
    _ => 0,
  }
}

fn raw_value(hand: &[Suit], jokers: bool) -> usize {
  hand
    .iter()
    .map(|suit| match suit {
      Suit::Jack => match jokers {
        false => Suit::Jack,
        true => Suit::Joker,
      },
      suit => *suit,
    })
    .enumerate()
    .map(|(index, suit)| usize::from(suit) * 100usize.pow((hand.len() - index) as u32))
    .sum()
}

fn build_suit_map(hand: &[Suit]) -> Vec<usize> {
  let occurences = hand
    .iter()
    .sorted()
    .group_by(|card| *card)
    .into_iter()
    .map(|(_, suit)| suit.count())
    .filter(|card| card > &1)
    .collect::<Vec<_>>();

  occurences
}

fn find_best_joker_replacement(hand: &[Suit]) -> Vec<Suit> {
  let hand = hand.to_vec();
  let not_jokers = hand.clone().into_iter().filter(|card| card != &Suit::Jack).collect::<Vec<_>>();

  if not_jokers.is_empty() {
    return hand;
  }

  let (hand, _) = not_jokers
    .iter()
    .map(|replacement| hand.clone().into_iter().map(|suit| if suit == Suit::Jack { *replacement } else { suit }).collect::<Vec<_>>())
    .map(|hand| (hand.clone(), hand_value(&build_suit_map(&hand))))
    .max_by_key(|(_, value)| *value)
    .unwrap();

  hand
}

fn compute_gains(hands: &[Hand]) -> i64 {
  hands.iter().sorted().enumerate().map(|(index, Hand(_, _, bid))| bid * (index as i64 + 1)).sum()
}

fn parse_hands(input: &str, jokers: bool) -> Vec<Hand> {
  util::read_file_lines(input)
    .into_iter()
    .flat_map(|hand| {
      hand
        .split_once(' ')
        .map(|(cards, bid)| (cards.chars().map(Suit::from).collect::<Vec<_>>(), parse::<i64>(bid)))
        .map(|(cards, bid)| {
          let value = raw_value(&cards, jokers);

          (cards, value, bid)
        })
        .map(|(cards, value, bid)| match jokers {
          false => Hand(hand_value(&build_suit_map(&cards)), value as i64, bid),
          true => Hand(hand_value(&build_suit_map(&find_best_joker_replacement(&cards))), value as i64, bid),
        })
    })
    .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> i64 {
  let hands = parse_hands(input, false);

  compute_gains(&hands)
}

pub fn part2(input: &str) -> i64 {
  let hands = parse_hands(input, true);

  compute_gains(&hands)
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1() {
    assert_eq!(super::part1("07_test.txt"), 6440);
  }

  #[test]
  fn part2() {
    assert_eq!(super::part2("07_test.txt"), 5905);
  }
}
