use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::iter::Iterator;

type Scalar = u8;

fn main() {
  let mut args = env::args();
  args.next();
  let mut cups = args
    .next()
    .unwrap()
    .bytes()
    .map(|v| v - b'0')
    .collect::<VecDeque<Scalar>>();

  for i in 0..100 {
    let current = cups.pop_front().unwrap();
    let next = cups.drain(0..3).collect::<Vec<_>>();
    cups.push_back(current);
    let destination = cups
      .iter()
      .map(|&v| {
        if v < current {
          current - v
        } else {
          current + 9 - v
        }
      })
      .enumerate()
      .sorted_by(|(_, a), (_, b)| Ord::cmp(a, b))
      .map(|(i, _)| i)
      .next()
      .unwrap()
      + 1;
    next.iter().rev().for_each(|&v| cups.insert(destination, v));
    println!("after round {}: {:?}", i+1, cups);
  }

  cups.rotate_left(
    cups
      .iter()
      .enumerate()
      .filter(|(_, &v)| v == 1)
      .map(|(i, _)| i)
      .next()
      .unwrap(),
  );

  println!("result: {:?}", String::from_utf8(cups.iter().skip(1).map(|&v| b'0' + v).collect::<Vec<_>>()).unwrap());
}
