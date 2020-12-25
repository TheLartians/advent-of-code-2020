use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::iter::Iterator;

type Scalar = u64;

fn transform(subject_number: Scalar, loop_size: usize) -> Scalar {
  let mut current = 1;
  for _ in 0..loop_size {
    current = (current * subject_number) % 20201227;
  }
  return current;
}

fn find_loop_size(public_key: Scalar) -> Option<usize> {
  let subject_number = 7;
  let mut current = 1;
  for i in 0..10000000000 {
    current = (current * subject_number) % 20201227;
    if current == public_key {
      return Some(i + 1);
    }
  }
  return None;
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let input = read_to_string(filename).unwrap();

  let public_keys = input
    .split('\n')
    .filter(|s| s.len() > 0)
    .map(|s| s.parse::<Scalar>().unwrap())
    .collect::<Vec<Scalar>>();
  let loop_sizes = public_keys
    .iter()
    .map(|&v| find_loop_size(v).unwrap())
    .collect::<Vec<_>>();
  println!("loop sizes: {:?}", loop_sizes);

  let encryption_keys = loop_sizes
    .iter()
    .zip(public_keys.iter().rev())
    .map(|(&i, &k)| transform(k, i))
    .collect::<Vec<_>>();

  println!("encryption keys: {:?}", encryption_keys);
}
