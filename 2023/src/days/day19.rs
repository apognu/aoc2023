use std::{
  array,
  collections::{HashMap, VecDeque},
  ops::{Index, IndexMut, RangeInclusive},
};

use itertools::Itertools;

use crate::util::{self, parse};

crate::tests!(19, (19114, 167409079868000));

#[derive(Debug, Default, Clone, Copy)]
struct Part {
  x: i64,
  m: i64,
  a: i64,
  s: i64,
}

#[derive(Debug, Clone, Copy)]
enum Rating {
  X,
  M,
  A,
  S,
}

impl From<char> for Rating {
  fn from(value: char) -> Self {
    match value {
      'x' => Rating::X,
      'm' => Rating::M,
      'a' => Rating::A,
      's' => Rating::S,
      _ => panic!("unexpected rating"),
    }
  }
}

impl Index<Rating> for Part {
  type Output = i64;

  fn index(&self, rating: Rating) -> &Self::Output {
    match rating {
      Rating::X => &self.x,
      Rating::M => &self.m,
      Rating::A => &self.a,
      Rating::S => &self.s,
    }
  }
}

impl IndexMut<Rating> for Part {
  fn index_mut(&mut self, rating: Rating) -> &mut Self::Output {
    match rating {
      Rating::X => &mut self.x,
      Rating::M => &mut self.m,
      Rating::A => &mut self.a,
      Rating::S => &mut self.s,
    }
  }
}

impl Index<Rating> for [RangeInclusive<i64>] {
  type Output = RangeInclusive<i64>;

  fn index(&self, rating: Rating) -> &Self::Output {
    match rating {
      Rating::X => &self[0],
      Rating::M => &self[1],
      Rating::A => &self[2],
      Rating::S => &self[3],
    }
  }
}

impl IndexMut<Rating> for [RangeInclusive<i64>] {
  fn index_mut(&mut self, rating: Rating) -> &mut Self::Output {
    match rating {
      Rating::X => &mut self[0],
      Rating::M => &mut self[1],
      Rating::A => &mut self[2],
      Rating::S => &mut self[3],
    }
  }
}

#[derive(Debug)]
struct Workflow {
  rules: Vec<Rule>,
}

impl Workflow {
  fn find_destination(&self, part: &Part) -> Destination {
    use Operation::*;

    for rule in &self.rules {
      match rule.condition {
        None => return rule.destination.clone(),
        Some(condition) => {
          let rating = &part[condition.rating];

          let result = match condition.op {
            GreaterThan => rating > &condition.value,
            Equal => rating == &condition.value,
            LessThan => rating < &condition.value,
          };

          if result {
            return rule.destination.clone();
          }
        }
      }
    }

    panic!("should never have no rule matching");
  }
}

#[derive(Debug, Clone)]
struct Rule {
  condition: Option<Condition>,
  destination: Destination,
}

#[derive(Debug, Clone, Copy)]
struct Condition {
  rating: Rating,
  op: Operation,
  value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
  GreaterThan,
  Equal,
  LessThan,
}

impl From<char> for Operation {
  fn from(value: char) -> Self {
    use Operation::*;

    match value {
      '>' => GreaterThan,
      '=' => Equal,
      '<' => LessThan,
      _ => panic!("unexpected operation"),
    }
  }
}

#[derive(Debug, Clone)]
enum Destination {
  Workflow(String),
  Accepted,
  Rejected,
}

impl From<&str> for Destination {
  fn from(value: &str) -> Self {
    use Destination::*;

    match value {
      "A" => Accepted,
      "R" => Rejected,
      name => Workflow(name.to_string()),
    }
  }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
  let input = util::read_file_lines(input).into_iter();
  let workflows = input.clone().take_while(|line| !line.is_empty()).collect::<Vec<_>>();
  let parts = input.skip(workflows.len() + 1).collect::<Vec<_>>();

  let workflows = workflows
    .into_iter()
    .map(|workflow| {
      let (name, rules) = workflow.split_once('{').unwrap();
      let rules = rules
        .trim_end_matches('}')
        .split(',')
        .flat_map(|rules| {
          rules
            .split(',')
            .map(|rule| {
              if !rule.contains(':') {
                let dest = Destination::from(rule);

                Rule { condition: None, destination: dest }
              } else {
                let (cond, dest) = rule.split_once(':').unwrap();

                let (rating, value) = cond.split(['<', '=', '>']).next_tuple().unwrap();
                let op = Operation::from(cond.trim_start_matches(rating).trim_end_matches(value).chars().next().unwrap());

                let rating: Rating = rating.chars().next().unwrap().into();
                let value = parse::<i64>(value);

                let destination = Destination::from(dest);

                Rule {
                  condition: Some(Condition { rating, op, value }),
                  destination,
                }
              }
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

      (name.to_string(), Workflow { rules })
    })
    .collect::<HashMap<_, _>>();

  let parts = parts
    .into_iter()
    .map(|part| {
      part.split(',').fold(Part::default(), |mut acc, spec| {
        let (rating, value) = spec.trim_start_matches('{').trim_end_matches('}').split_once('=').unwrap();
        let rating: Rating = rating.chars().next().unwrap().into();

        acc[rating] = parse::<i64>(value);
        acc
      })
    })
    .collect::<Vec<_>>();

  (workflows, parts)
}

pub fn part1(input: &str) -> i64 {
  let (workflows, parts) = parse_input(input);
  let mut queue: VecDeque<(String, Part)> = VecDeque::default();
  let mut accepted: Vec<Part> = vec![];

  for part in parts {
    queue.push_back(("in".to_string(), part));
  }

  while let Some((name, part)) = queue.pop_front() {
    let workflow = workflows.get(&name).unwrap();
    let destination = workflow.find_destination(&part);

    match destination {
      Destination::Accepted => accepted.push(part),
      Destination::Workflow(rule) => queue.push_back((rule, part)),
      _ => {}
    }
  }

  accepted.into_iter().map(|part| part.x + part.m + part.a + part.s).sum::<i64>()
}

fn find_accepted_ranges(workflows: &HashMap<String, Workflow>, dest: Destination, mut ranges: [RangeInclusive<i64>; 4]) -> i64 {
  use Operation::*;

  let mut total = 0;

  match dest {
    Destination::Rejected => return 0,
    Destination::Accepted => return ranges.iter().map(|range| range.end() - range.start() + 1).product(),

    Destination::Workflow(workflow) => {
      for rule in &workflows.get(&workflow).unwrap().rules {
        match rule.condition {
          None => total += find_accepted_ranges(workflows, rule.destination.clone(), ranges.clone()),

          Some(condition) => {
            let mut selection = ranges.clone();
            let (lrange, rrange) = (ranges[condition.rating].start(), ranges[condition.rating].end());

            if condition.op == LessThan && *lrange < condition.value {
              selection[condition.rating] = *lrange..=condition.value - 1;

              total += find_accepted_ranges(workflows, rule.destination.clone(), selection);

              if condition.value < *rrange {
                ranges[condition.rating] = condition.value..=*rrange;
              }
            } else if *rrange > condition.value {
              selection[condition.rating] = condition.value + 1..=*rrange;

              total += find_accepted_ranges(workflows, rule.destination.clone(), selection);

              if *lrange <= condition.value {
                ranges[condition.rating] = *lrange..=condition.value;
              }
            }
          }
        }
      }
    }
  }

  total
}

pub fn part2(input: &str) -> i64 {
  let (workflows, _) = parse_input(input);

  find_accepted_ranges(&workflows, Destination::Workflow("in".to_string()), array::from_fn(|_| 1..=4000))
}
