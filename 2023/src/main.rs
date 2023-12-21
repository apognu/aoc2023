#![allow(dead_code)]

mod days;
#[macro_use]
mod util;

use std::{process, time::Instant};

use aoc_macros::generate_days;
use argparse::{ArgumentParser, StoreOption, StoreTrue};

use crate::util::Options;

fn main() {
  let days = generate_days!();

  let (mut day, mut part): (Option<usize>, Option<usize>) = (None, None);
  let (mut test, mut timings) = (false, false);

  {
    let mut args = ArgumentParser::new();

    args.refer(&mut test).add_option(&["-t", "--test"], StoreTrue, "run with test input");
    args.refer(&mut timings).add_option(&["--timings"], StoreTrue, "run with timings");
    args.refer(&mut day).add_argument("DAY", StoreOption, "day of the month");
    args.refer(&mut part).add_argument("PART", StoreOption, "puzzle part");

    args.parse_args_or_exit();
  }

  if let Some(day) = day {
    let part = part.unwrap_or(0);

    match days.get(day - 1) {
      Some(funcs) => {
        match part {
          1 => execute(funcs.0, day, part, test, timings),
          2 => execute(funcs.1, day, part, test, timings),

          _ => {
            execute(funcs.0, day, 1, test, timings);
            execute(funcs.1, day, 2, test, timings);
          }
        }

        process::exit(0);
      }

      None => {
        eprintln!("Unknown day, come back later! o7");
        process::exit(1);
      }
    }
  }

  for (index, funcs) in days.iter().enumerate() {
    execute(funcs.0, index + 1, 1, test, timings);
    execute(funcs.1, index + 1, 2, test, timings);
  }
}

fn execute(func: fn(&str, Options) -> i64, day: usize, part: usize, test: bool, timings: bool) {
  let input = &util::input_file(day, part, test);
  let before = Instant::now();
  let result = func(input, None);

  print!("D{day:0>2}P{part:0>2}: {result} ");

  if timings {
    print!("({:?})", Instant::now().duration_since(before));
  }

  println!();
}
