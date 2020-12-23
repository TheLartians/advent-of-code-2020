use std::env;
use std::iter::Iterator;

type Cup = usize;
type Cups = Vec<Cup>;

struct CupTraverse<'a> {
  curr: usize,
  cups: &'a Cups,
}

fn cup_traverse<'a>(cups: &'a Cups, current_value: usize) -> CupTraverse<'a> {
  return CupTraverse {
    curr: current_value,
    cups: cups,
  };
}

impl Iterator for CupTraverse<'_> {
  type Item = usize;
  fn next(&mut self) -> Option<Self::Item> {
    self.curr = self.cups[self.curr];
    return Some(self.curr);
  }
}

fn main() {
  let mut args = env::args();
  args.next();
  let input = args
    .next()
    .unwrap()
    .bytes()
    .map(|v| (v - b'0' - 1) as Cup)
    .collect::<Vec<_>>();

  let cup_count = 1000000;
  let rounds = 10000000;
  let mut cups = Vec::new();
  const pick_len: usize = 3;

  for i in 0..cup_count {
    cups.push(i + 1);
  }

  let mut current = cups.len() - 1;
  for &v in input.iter() {
    cups[current] = v;
    current = v;
  }
  cups[current] = if input.len() < cup_count {
    input.len() % cup_count
  } else {
    input[0]
  };

  let mut current = input[0];
  for _round in 0..rounds {
    let mut destination = (current + cup_count - 1) % cup_count;
    while cup_traverse(&cups, current)
      .take(pick_len)
      .any(|v| v == destination)
    {
      destination = (destination + cup_count - 1) % cup_count;
    }
    let final_pick = cup_traverse(&cups, current).take(pick_len).last().unwrap();
    let after_pick = cups[final_pick];
    let after_dest = cups[destination];
    cups[destination] = cups[current];
    cups[final_pick] = after_dest;
    cups[current] = after_pick;
    current = cups[current];
  }

  println!(
    "the result is {}",
    cup_traverse(&cups, 0)
      .take(2)
      .map(|c| c + 1)
      .fold(1, |a, b| a * b)
  );
}
