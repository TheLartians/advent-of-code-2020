use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::mem;
use std::string::String;

fn main() {
  let mut args = env::args();
  args.next();
  let input = args
    .next()
    .unwrap()
    .split(',')
    .map(|s| s.parse::<Scalar>().unwrap())
    .collect::<Vec<Scalar>>();

  type Scalar = usize;

  let mut state: HashMap<Scalar, Scalar> = input[..input.len() - 1]
    .iter()
    .enumerate()
    .map(|(a, &b)| (b, a))
    .collect();
  let mut current = *input.last().unwrap();

  for i in state.len()..2019 {
    let next;
    if let Some(j) = state.get(&current) {
      next = i - j;
    } else {
      next = 0;
    }
    state.insert(current, i);
    current = next;
  }

  println!("the 2020th number is {}", current);
}
