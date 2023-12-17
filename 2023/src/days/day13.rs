use crate::util::{self, transpose};

crate::tests!(13, (405, 400));

type Grid = Vec<Vec<bool>>;

fn parse_field(input: &str) -> Vec<Grid> {
  let input = util::read_file_lines(input);

  let fields = input
    .iter()
    .map(|line| {
      line
        .chars()
        .map(|symbol| match symbol {
          '#' => true,
          '.' => false,
          _ => panic!("unexpected character"),
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  fields.split(|row| row.is_empty()).map(|field| field.to_vec()).collect::<Vec<_>>()
}

fn find_mirror_point(field: &Grid, max: usize) -> i64 {
  for i in 0..field.len() {
    if i >= field.len() - 1 {
      break;
    }

    let root_differences = find_difference_count(&field[i], &field[i + 1]);

    if root_differences <= max {
      let mut left = i as isize - 1;
      let mut right = i as isize + 2;
      let mut differences = 0;

      loop {
        if left < 0 || right > field.len() as isize - 1 {
          if (root_differences + differences) == max {
            return i as i64 + 1;
          }

          break;
        }

        differences += find_difference_count(&field[left as usize], &field[right as usize]);

        left -= 1;
        right += 1;
      }
    }
  }

  0
}

fn find_difference_count(a: &[bool], b: &[bool]) -> usize {
  a.iter().enumerate().filter(|(index, left)| b.get(*index).unwrap() != *left).count()
}

pub fn part1(input: &str) -> i64 {
  let fields = parse_field(input);

  fields.into_iter().map(|field| (100 * find_mirror_point(&field, 0)) + find_mirror_point(&transpose(field), 0)).sum()
}

pub fn part2(input: &str) -> i64 {
  let fields = parse_field(input);

  fields.into_iter().map(|field| (100 * find_mirror_point(&field, 1)) + find_mirror_point(&transpose(field), 1)).sum()
}
