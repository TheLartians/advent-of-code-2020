use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::iter::Iterator;

type Scalar = usize;

fn parse_cards(input: &str) -> VecDeque<Scalar> {
  return input
    .split('\n')
    .skip(1)
    .filter(|s| s.len() > 0)
    .map(|s| s.parse().unwrap())
    .collect();
}

fn recursive_combat(
  deck1: &mut VecDeque<Scalar>,
  deck2: &mut VecDeque<Scalar>,
  game: usize,
) -> bool {
  let mut cache: HashSet<(Vec<Scalar>, Vec<Scalar>)> = HashSet::new();

  while ![&deck1, &deck2].iter().any(|d| d.len() == 0) {
    let key = (
      deck1.iter().map(|&v| v).collect(),
      deck2.iter().map(|&v| v).collect(),
    );
    if cache.contains(&key) {
      deck2.clear();
    } else {
      cache.insert(key);
      let p1 = deck1.pop_front().unwrap();
      let p2 = deck2.pop_front().unwrap();

      let p1_wins = if deck1.len() >= p1 && deck2.len() >= p2 {
        recursive_combat(
          &mut deck1.iter().take(p1).map(|&v| v).collect(),
          &mut deck2.iter().take(p2).map(|&v| v).collect(),
          game + 1,
        )
      } else {
        p1 > p2
      };

      if p1_wins {
        deck1.push_back(p1);
        deck1.push_back(p2);
      } else {
        deck2.push_back(p2);
        deck2.push_back(p1);
      }
    }
  }

  return deck2.len() == 0;
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

  recursive_combat(&mut deck1, &mut deck2, 0);

  println!(
    "the final score result is {}",
    [&deck1, &deck2]
      .iter()
      .map(|&v| v)
      .flatten()
      .rev()
      .enumerate()
      .map(|(i, &v)| (i as Scalar + 1) * v)
      .fold(0, |a, b| a + b)
  );
}
