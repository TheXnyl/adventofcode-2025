use adventofcode_2025::prelude::*;
use std::collections::{HashMap, hash_map};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
  x: i64,
  y: i64,
  z: i64,
}

#[derive(Debug)]
struct UnionFind {
  root: HashMap<Pos, Pos>,
}

impl UnionFind {
  fn new(data: &[Pos]) -> UnionFind {
    let mut root: HashMap<Pos, Pos> = HashMap::new();
    for value in data {
      root.insert(*value, *value);
    }

    return UnionFind { root };
  }

  fn find(&mut self, x: Pos) -> Pos {
    let parent = self.root[&x];

    if parent != x {
      let root = self.find(parent);
      self.root.insert(x, root);
      return root;
    }

    return x;
  }

  fn compress(&mut self) {
    for (child, _) in self.root.clone() {
      self.find(child);
    }
  }

  fn union(&mut self, a: Pos, b: Pos) {
    let root_a = self.find(a);
    let root_b = self.find(b);

    if root_a != root_b {
      self.root.insert(root_a, root_b);
    }
  }
}

fn dist_sq(a: &Pos, b: &Pos) -> i64 {
  let dx = a.x - b.x;
  let dy = a.y - b.y;
  let dz = a.z - b.z;
  return dx * dx + dy * dy + dz * dz;
}

fn get_two_closest(positions: &Vec<Pos>) -> (usize, usize) {
  let mut best_i = 0;
  let mut best_j = 1;
  let mut best_dist = dist_sq(&positions[0], &positions[1]);

  for i in 0..positions.len() {
    for j in (i + 1)..positions.len() {
      let d = dist_sq(&positions[i], &positions[j]);
      if d < best_dist {
        best_dist = d;
        best_i = i;
        best_j = j;
      }
    }
  }

  return (best_i, best_j);
}

fn main() {
  // Part 1
  let reader = get_reader("day08.txt");

  let mut positions: Vec<Pos> = vec![];
  for line in reader.lines() {
    let line = line.unwrap();
    let _pos: Vec<i64> = line
      .trim()
      .split(',')
      .map(|x| x.parse::<i64>().unwrap())
      .collect();

    positions.push(Pos {
      x: _pos[0],
      y: _pos[1],
      z: _pos[2],
    });
  }

  let mut pairs: Vec<(Pos, Pos)> = vec![];
  for i in 0..positions.len() {
    for j in i + 1..positions.len() {
      pairs.push((positions[i], positions[j]));
    }
  }

  pairs.sort_by_key(|(a, b)| dist_sq(a, b));

  let mut uf = UnionFind::new(&positions);

  for (a, b) in pairs.iter().take(1000) {
    uf.union(*a, *b);
  }

  // uf.compress_all();

  let mut counts: HashMap<Pos, usize> = HashMap::new();
  for pos in &positions {
    let root = uf.find(*pos);
    *counts.entry(root).or_insert(0) += 1;
  }

  let mut sizes: Vec<usize> = counts.values().cloned().collect();
  sizes.sort_unstable();
  sizes.reverse();

  let total: usize = sizes.iter().take(3).product();
  println!("Part 1: {}", total);

  // Part 2
  let reader = get_reader("day08.txt");

  let mut positions: Vec<Pos> = vec![];
  for line in reader.lines() {
    let line = line.unwrap();
    let _pos: Vec<i64> = line
      .trim()
      .split(',')
      .map(|x| x.parse::<i64>().unwrap())
      .collect();

    positions.push(Pos {
      x: _pos[0],
      y: _pos[1],
      z: _pos[2],
    });
  }

  let mut pairs: Vec<(Pos, Pos)> = vec![];
  for i in 0..positions.len() {
    for j in i + 1..positions.len() {
      pairs.push((positions[i], positions[j]));
    }
  }

  pairs.sort_by_key(|(a, b)| dist_sq(a, b));

  let mut uf = UnionFind::new(&positions);
  let mut num_circuits = positions.len();
  for (a, b) in pairs {
    let root_a = uf.find(a);
    let root_b = uf.find(b);

    if root_a != root_b {
      uf.union(a, b);
      num_circuits -= 1;

      if num_circuits == 1 {
        println!("Part 2: {}", a.x * b.x);
        break;
      }
    }
  }
}
