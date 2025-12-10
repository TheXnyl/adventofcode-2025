use adventofcode_2025::prelude::*;
use std::collections::{HashMap, hash_map};

struct UnionFind {
  root: HashMap<u32, u32>,
}

impl UnionFind {
  fn new(data: &[u32]) -> UnionFind {
    let mut root: HashMap<u32, u32> = HashMap::new();
    for value in data {
      root.insert(*value, *value);
    }

    return UnionFind { root };
  }

  fn find_compress(&mut self, x: u32) -> u32 {
    let parent = self.root[&x];

    if parent != x {
      let root = self.find_compress(parent);
      self.root.insert(x, root);
      return root;
    }

    return x;
  }

  fn compress_all(&mut self) {
    for (child, _) in self.root.clone() {
      self.find_compress(child);
    }
  }

  fn union(&mut self, a: u32, b: u32) {
    let root_a = self.find_compress(a);
    let root_b = self.find_compress(b);

    if root_a != root_b {
      self.root.insert(root_a, root_b);
    }
  }
}

fn main() {
  let data = [2, 3, 4, 5, 6];
  let pairs = [(2, 3), (3, 4), (5, 6)];

  let mut uf = UnionFind::new(&data);
  for (a, b) in pairs {
    uf.union(a, b);
  }

  uf.compress_all();

  for (child, parent) in uf.root.clone() {
    println!("{:?}, {:?}", child, parent);
  }
}
