use std::{fs, path::Path};

pub fn input_file(day: usize, part: usize, test: bool) -> String {
  match test {
    true => match Path::new(&format!("data/{day:0>2}_{part}_test.txt")).exists() {
      true => format!("{day:0>2}_{part}_test.txt"),
      false => format!("{day:0>2}_test.txt"),
    },

    false => format!("{day:0>2}.txt"),
  }
}

pub fn read_file_lines(input: &str) -> Vec<String> {
  fs::read_to_string(format!("data/{input}")).unwrap().lines().map(String::from).collect()
}
