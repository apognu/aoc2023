use crate::util::{self, transpose};

crate::tests!(14, (136, 64));

type Grid = Vec<Vec<char>>;

fn parse_platform(input: &str) -> Grid {
  util::read_file_lines(input).iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn rotate(matrix: Grid) -> Grid {
  let mut matrix = transpose(matrix);

  matrix.iter_mut().for_each(|row| row.reverse());
  matrix
}

fn tilt(mut platform: Grid) -> Grid {
  for col in 0..platform[0].len() {
    let mut min = 0;

    for row in 0..platform.len() {
      match platform[row][col] {
        '#' => min = row + 1,
        'O' => {
          if row > min {
            platform[min][col] = 'O';
            platform[row][col] = '.';
          }

          min += 1;
        }
        '.' => {}
        _ => panic!("unexpected character"),
      }
    }
  }

  platform
}

fn count(platform: &Grid) -> i64 {
  let mut score = 0;

  for col in 0..platform[0].len() {
    for (index, row) in (0..platform.len()).enumerate() {
      if platform[row][col] == 'O' {
        score += platform.len() - index;
      }
    }
  }

  score as i64
}

pub fn part1(input: &str) -> i64 {
  count(&tilt(parse_platform(input)))
}

pub fn part2(input: &str) -> i64 {
  let max = 1_000_000_000;
  let mut platform = parse_platform(input);
  let mut cycle = 0;

  let mut seen: Vec<Grid> = Vec::new();

  while cycle < max {
    for _ in 0..4 {
      platform = rotate(tilt(platform));
    }

    if let Some(memoized) = seen.iter().position(|saved| saved == &platform) {
      let seen = &seen[memoized..];

      return count(&seen[(max - cycle - 1) % seen.len()]);
    }

    seen.push(platform.clone());

    cycle += 1;
  }

  panic!("should not be reached");
}
