use adventofcode_2025::prelude::*;

fn main() {
  // Part 1
  let reader = get_reader("day01.txt");

  let mut password: u32 = 0;
  let mut arrow = 50;
  for line in reader.lines() {
    let rotation = line.unwrap();
    let right = rotation.chars().next().unwrap() == 'R';
    let amount: i32 = rotation[1..].parse().unwrap();

    arrow += if right { amount } else { -amount };
    arrow %= 100;
    password += (arrow == 0) as u32;
  }

  println!("Part 1: {}", password);

  // Part 2

  let reader = get_reader("day01.txt");

  let mut password: u32 = 0;
  let mut arrow = 50;
  for line in reader.lines() {
    let rotation = line.unwrap();
    let right = rotation.chars().next().unwrap() == 'R';
    let amount: i32 = rotation[1..].parse().unwrap();

    for _ in 0..amount {
      if right {
        arrow += 1;
      } else {
        arrow -= 1;
      }

      if arrow < 0 {
        arrow = 99;
      } else if arrow > 99 {
        arrow = 0;
      }

      password += (arrow == 0) as u32;
    }
  }

  println!("Part 2: {}", password);
}
