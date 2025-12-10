use adventofcode_2025::prelude::*;

fn main() {
  // Part 1
  let reader = get_reader("day06.txt");

  let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

  let op_line = lines.last().unwrap();
  let operators: Vec<char> = op_line
    .split_whitespace()
    .map(|s| s.chars().next().unwrap())
    .collect();

  let number_lines = &lines[..lines.len() - 1];
  let numbers: Vec<Vec<u64>> = number_lines
    .iter()
    .map(|line| {
      line
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
    })
    .collect();

  let rows = numbers.len();
  let cols = numbers[0].len();

  let mut total = 0u64;

  for col in 0..cols {
    let op = operators[col];
    let mut acc = match op {
      '+' => 0,
      '*' => 1,
      _ => unreachable!(),
    };

    for row in 0..rows {
      let val = numbers[row][col];

      acc = match op {
        '+' => acc + val,
        '*' => acc * val,
        _ => unreachable!(),
      };
    }

    total += acc;
  }

  println!("{:?}", total);

  // Part 2
  let reader = get_reader("day06.txt");
  for line in reader.lines() {}
}
