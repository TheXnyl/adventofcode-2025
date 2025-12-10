use adventofcode_2025::prelude::*;

#[derive(Debug)]
struct Range {
  start: u64,
  end: u64,
}

impl Range {
  fn in_range(&self, value: u64) -> bool {
    return value >= self.start && value <= self.end;
  }
}

fn main() {
  // Part 1
  let reader = get_reader("day05.txt");

  let mut ranges: Vec<Range> = Vec::new();
  let mut ingredients: Vec<u64> = Vec::new();

  let mut getting_ranges = true;
  for line in reader.lines() {
    let line = line.unwrap();
    if line == "" {
      getting_ranges = false;
      continue;
    }

    if getting_ranges {
      let dash_index = line.find('-').unwrap();

      let range = Range {
        start: line[..dash_index].parse().unwrap(),
        end: line[dash_index + 1..].parse().unwrap(),
      };

      ranges.push(range);
    } else {
      ingredients.push(line.parse().unwrap());
    }
  }

  let mut total = 0;
  for ingredient in &ingredients {
    for range in &ranges {
      if range.in_range(*ingredient) {
        total += 1;
        break;
      }
    }
  }

  // println!("{}", total);

  // Part 2
  let reader = get_reader("day05.txt");

  let mut ranges: Vec<Range> = Vec::new();
  for line in reader.lines() {
    let line = line.unwrap();
    if line == "" {
      break;
    }

    let dash_index = line.find('-').unwrap();

    let range = Range {
      start: line[..dash_index].parse().unwrap(),
      end: line[dash_index + 1..].parse().unwrap(),
    };

    ranges.push(range);
  }

  let mut total = 0;
  ranges.sort_by_key(|r| r.start);

  let mut merged: Vec<Range> = Vec::new();
  for range in ranges {
    if let Some(last) = merged.last_mut() {
      if range.start <= last.end + 1 {
        last.end = last.end.max(range.end);
        continue;
      }
    }
    merged.push(range);
  }


  for range in merged
  {
    total += range.end - range.start + 1;
  }

  println!("{}", total);
}
