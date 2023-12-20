use std::collections::{HashMap, VecDeque};

use num::integer::lcm;

use crate::util;

crate::tests!(20, (0, 0));

#[derive(Debug, Clone)]
enum Device {
  Broadcaster { name: String, outputs: Vec<String> },
  FlipFlop { name: String, state: bool, outputs: Vec<String> },
  Conjunction { name: String, inputs: HashMap<String, Pulse>, outputs: Vec<String> },
}

impl Device {
  fn outputs(&self) -> &[String] {
    use Device::*;

    match self {
      Broadcaster { outputs, .. } => outputs,
      FlipFlop { outputs, .. } => outputs,
      Conjunction { outputs, .. } => outputs,
    }
  }

  fn send(&mut self, pulse: Pulse, from: &str, queue: &mut VecDeque<(String, String, Pulse)>) {
    use Device::*;

    match self {
      Broadcaster { name, outputs } => {
        for output in outputs {
          queue.push_back((name.to_string(), output.to_string(), pulse));
        }
      }

      FlipFlop { name, state, outputs } => match pulse {
        Pulse::High => {}
        Pulse::Low => {
          let pulse = match state {
            false => Pulse::High,
            true => Pulse::Low,
          };

          for output in outputs {
            queue.push_back((name.to_string(), output.to_string(), pulse));
          }

          *state = !*state;
        }
      },

      Conjunction { name, inputs, outputs } => {
        inputs.insert(from.to_string(), pulse);

        let pulse = match inputs.iter().all(|(_, pulse)| pulse == &Pulse::High) {
          true => Pulse::Low,
          false => Pulse::High,
        };

        for output in outputs {
          queue.push_back((name.clone(), output.clone(), pulse));
        }
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
  High,
  Low,
}

fn parse_network(input: &str) -> HashMap<String, Device> {
  use Device::*;

  let mut network = util::read_file_lines(input)
    .iter()
    .map(|line| {
      let (label, outputs) = line.split_once(" -> ").unwrap();
      let outputs = outputs.split(", ").map(String::from).collect::<Vec<_>>();

      let (name, device) = if label == "broadcaster" {
        (label.to_string(), Broadcaster { name: label.to_string(), outputs })
      } else {
        let name = label[1..].to_string();

        let device = match label.chars().next().unwrap() {
          '%' => FlipFlop {
            name: name.clone(),
            state: false,
            outputs,
          },

          '&' => Conjunction {
            name: name.clone(),
            inputs: HashMap::default(),
            outputs,
          },

          _ => panic!("unexpected device type"),
        };

        (name, device)
      };

      (name, device)
    })
    .collect::<HashMap<_, _>>();

  for (name, device) in network.clone() {
    for output in device.outputs() {
      if let Some(Conjunction { inputs, .. }) = network.get_mut(output) {
        inputs.insert(name.clone(), Pulse::Low);
      }
    }
  }

  network
}

pub fn part1(input: &str) -> i64 {
  let mut network = parse_network(input);
  let (mut highs, mut lows) = (0, 0);

  for _ in 1..=1000 {
    let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::default();
    queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

    while let Some((from, target, pulse)) = queue.pop_front() {
      match pulse {
        Pulse::High => highs += 1,
        Pulse::Low => lows += 1,
      }

      if let Some(device) = network.get_mut(&target) {
        device.send(pulse, &from, &mut queue);
      }
    }
  }

  highs * lows
}

pub fn part2(input: &str) -> i64 {
  use Device::*;

  let mut network = parse_network(input);
  let mut tracker: HashMap<String, i64> = HashMap::default();

  let mut presses = 1;

  let previous = network
    .iter()
    .find(|(_, device)| device.outputs().contains(&String::from("rx")))
    .map(|(device, _)| device.clone())
    .unwrap();

  loop {
    let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::default();
    queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

    while let Some((from, target, pulse)) = queue.pop_front() {
      if let Some(device) = network.get_mut(&target) {
        device.send(pulse, &from, &mut queue);

        // mg is the only node outputing to rx, and it is a conjunction, so let
        // us track the iteration at which its states change and LCM them.
        if let Conjunction { name, inputs, .. } = &device {
          if name == &previous {
            for (input, last_pulse) in inputs {
              if !tracker.contains_key(input) && last_pulse == &Pulse::High {
                tracker.insert(input.to_string(), presses);
              }
            }

            if tracker.len() == inputs.len() {
              return tracker.values().fold(1, |acc, press| lcm(acc, *press));
            }
          }
        }
      }
    }

    presses += 1;
  }
}
