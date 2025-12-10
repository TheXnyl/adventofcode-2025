use adventofcode_2025::prelude::*;

struct Range(u64, u64);

fn main() {
  // Part 2
  let reader = get_reader("day02.txt");
  let line = reader.lines().next().unwrap().unwrap();

  let ranges: Vec<Range> = line
    .split(',')
    .map(|part| {
      let bounds: Vec<u64> = part.split('-').map(|x| x.parse().unwrap()).collect();
      Range(bounds[0], bounds[1])
    })
    .collect();

  for range in &ranges {
    println!("{} - {}", range.0, range.1);
  }

  let mut total: u64 = 0;

  for Range(start, end) in ranges {
    for num in start..=end {
      let num_str = num.to_string();
      let len = num_str.len();

      for seq_len in 1..=num_str.len() / 2 {
        let repeats = len / seq_len;
        let seq = &num_str[..seq_len];
        if seq.repeat(repeats) == num_str {
          total += num;
          break;
        }
      }
    }
  }

  println!("{}", total);
}
