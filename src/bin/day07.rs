use adventofcode_2025::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
  row: usize,
  col: usize,
}

type Input = Vec<Pos>;

fn exists_right(input: &Input, pos: &Pos) -> Option<Pos> {
  input
    .iter()
    .filter(|p| p.row > pos.row && p.col == pos.col + 1)
    .min_by_key(|p| p.row)
    .copied()
}

fn exists_left(input: &Input, pos: &Pos) -> Option<Pos> {
  input
    .iter()
    .filter(|p| p.row > pos.row && p.col == pos.col - 1)
    .min_by_key(|p| p.row)
    .copied()
}

fn main() {
  // Part 1
  let reader = get_reader("day07.txt");

  let mut input: Input = vec![];

  for (i, line) in reader.lines().enumerate() {
    for (j, char) in line.unwrap().chars().enumerate() {
      if char != '.' {
        input.push(Pos {
          row: i + 1,
          col: j + 1,
        });
      }
    }
  }

  let mut total = 1;
  let mut current = vec![input[1]];

  if (input[0].col != input[1].col) {
    unreachable!();
  }

  let mut visited: HashSet<Pos> = HashSet::new();

  loop {
    let mut next: Vec<Pos> = vec![];

    for pos in &current {
      let right = exists_right(&input, pos);
      let left = exists_left(&input, pos);

      if let Some(pos) = right {
        if visited.insert(pos) {
          next.push(pos);
          total += 1;
        }
      }
      if let Some(pos) = left {
        if visited.insert(pos) {
          next.push(pos);
          total += 1;
        }
      }
    }

    if next.is_empty() {
      break;
    }

    current = next;
  }

  println!("{}", total * 2);

  // Part 2
  let reader = get_reader("day07.txt");
  for line in reader.lines() {}
}
