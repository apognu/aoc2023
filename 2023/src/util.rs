use std::{fmt::Debug, fs, path::Path, str::FromStr};

#[macro_export]
macro_rules! tests {
  ($day:literal, ($result1:literal, $result2:literal)) => {
    #[cfg(test)]
    mod tests {
      #[test]
      fn part1() {
        assert_eq!(super::part1(&$crate::util::input_file($day, 1, true)), $result1);
      }

      #[test]
      fn part2() {
        assert_eq!(super::part2(&$crate::util::input_file($day, 2, true)), $result2);
      }
    }
  };
}

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

pub fn parse<T>(value: &str) -> T
where
  T: FromStr,
  <T as FromStr>::Err: Debug,
{
  value.parse::<T>().unwrap()
}

pub fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
where
  T: Clone,
{
  (0..matrix[0].len()).map(|index| matrix.iter().map(|row| row[index].clone()).collect::<Vec<_>>()).collect()
}
