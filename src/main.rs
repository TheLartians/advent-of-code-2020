use itertools::Itertools;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::iter::Iterator;

fn parse_cards(input: &str) -> VecDeque<i64> {
  return input
    .split('\n')
    .skip(1)
    .filter(|s| s.len() > 0)
    .map(|s| s.parse().unwrap())
    .collect();
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let (mut deck1, mut deck2) = fs::read_to_string(&filename)
    .unwrap()
    .split("\n\n")
    .map(|s| parse_cards(&s))
    .next_tuple()
    .unwrap();

  while ![&deck1, &deck2].iter().any(|d| d.len() == 0) {
    let p1 = deck1.pop_front().unwrap();
    let p2 = deck2.pop_front().unwrap();
    if p1 > p2 {
      deck1.push_back(p1);
      deck1.push_back(p2);
    } else {
      deck2.push_back(p2);
      deck2.push_back(p1);
    }
  }

  println!(
    "the final score result is {}",
    [&deck1, &deck2]
      .iter()
      .map(|&v| v)
      .flatten()
      .rev()
      .enumerate()
      .map(|(i, &v)| (i as i64 + 1) * v)
      .fold(0, |a, b| a + b)
  );
}
