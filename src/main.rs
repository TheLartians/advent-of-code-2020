use std::collections::HashMap;
use std::env;

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

  let n = 30000000;

  for i in state.len()..n - 1 {
    let next = match state.get(&current) {
      Some(j) => i-j,
      None => 0,
    };
    state.insert(current, i);
    current = next;
  }

  println!("the {} number is {}", n, current);
}
