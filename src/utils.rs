use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn get_reader(file: &str) -> BufReader<File> {
  let file = File::open(format!("input/{file}")).expect("File doesn't exist");
  return BufReader::new(file);
}
