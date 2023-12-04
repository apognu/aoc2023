#![allow(dead_code)]

mod days;
mod util;

use std::{env, process};

use aoc_macros::generate_days;

fn main() {
  let days = generate_days!(4);

  let mut args: Vec<_> = env::args().collect();
  let mut test = false;

  if args.len() < 2 {
    eprintln!("USAGE: cargo run -- [-t] DAY [PART]");
    process::exit(1);
  }

  if args.get(1).unwrap().as_str() == "-t" {
    test = true;

    args.remove(1);
  }

  if let Some(Ok(day)) = args.get(1).map(|day| day.parse::<usize>()) {
    let part = args.get(2).and_then(|day| day.parse::<usize>().ok()).unwrap_or(0);

    match days.get(day - 1) {
      Some(funcs) => {
        match part {
          1 => println!("Day {}, part 1: {}", day, funcs.0(&util::input_file(day, part, test))),
          2 => println!("Day {}, part 2: {}", day, funcs.1(&util::input_file(day, part, test))),
          _ => {
            println!("Day {}, part 1: {}", day, funcs.0(&util::input_file(day, 1, test)));
            println!("Day {}, part 2: {}", day, funcs.1(&util::input_file(day, 2, test)));
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

  eprintln!("Could not understand which day you are looking for.");
  process::exit(1);
}
