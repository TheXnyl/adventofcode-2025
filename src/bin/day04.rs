use adventofcode_2025::prelude::*;

#[derive(Debug)]
struct Pos {
  row: i32,
  col: i32,
}

fn check(chars: &Vec<char>, pos: &Pos, rows: i32, cols: i32) -> bool {
  if pos.row < 0 || pos.row >= rows || pos.col < 0 || pos.col >= cols {
    return false;
  }

  let index = get_index(pos, cols);
  chars[index as usize] == '@'
}

fn get_pos(index: i32, cols: i32) -> Pos {
  return Pos {
    row: index / cols,
    col: index % cols,
  };
}

fn get_index(pos: &Pos, cols: i32) -> i32 {
  return pos.row * cols + pos.col;
}

fn main() {
  // Part 1
  let reader = get_reader("day04.txt");
  let mut chars: Vec<char> = Vec::new();
  let mut cols: i32 = 0;
  let mut rows: i32 = 0;
  for line in reader.lines() {
    let line = line.unwrap();
    cols = line.len() as i32;
    chars.extend(line.chars());
    rows += 1;
  }

  let mut total: u32 = 0;

  loop {
    let mut b_removed = false;
    for i in 0..chars.len() {
      if chars[i] != '@' {
        continue;
      }

      let i = i as i32;

      let current_pos = get_pos(i, cols);
      let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
      ];

      let mut neighbors = 0;
      for (row_offset, col_offset) in offsets.iter() {
        let pos = Pos {
          row: current_pos.row + row_offset,
          col: current_pos.col + col_offset,
        };
        if check(&chars, &pos, rows, cols) {
          neighbors += 1;
        }
      }

      if neighbors < 4 {
        chars[i as usize] = '.';
        total += 1;
        b_removed = true;
      }
    }

    if !b_removed {
      break;
    }
  }

  println!("{}", total);

  // Part 2
  let reader = get_reader("day04.txt");
  let mut chars: Vec<char> = Vec::new();
  let mut line_len: usize;
  for line in reader.lines() {
    let line = line.unwrap();
    let line_len = line.len();
    chars.extend(line.chars());
  }
}
