use std::env;
use std::fs::read_to_string;
use std::iter::Iterator;

type Scalar = u64;

struct Transformer {
  current: Scalar,
  subject_number: Scalar,
}

impl Iterator for Transformer {
  type Item = Scalar;

  fn next(&mut self) -> Option<Self::Item> {
    self.current = (self.current * self.subject_number) % 20201227;
    return Some(self.current);
  }
}

fn transform(subject_number: Scalar) -> Transformer {
  return Transformer {
    current: 1,
    subject_number: subject_number,
  };
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let input = read_to_string(filename).unwrap();

  let public_keys: Vec<Scalar> = input
    .split('\n')
    .filter(|s| s.len() > 0)
    .map(|s| s.parse::<Scalar>().unwrap())
    .collect();

  let loop_sizes: Vec<_> = public_keys
    .iter()
    .map(|p| {
      transform(7)
        .enumerate()
        .filter(|(_, v)| v == p)
        .next()
        .map(|(i, _)| i)
        .unwrap()
    })
    .collect();

  let encryption_keys: Vec<_> = public_keys
    .iter()
    .rev()
    .zip(loop_sizes.iter())
    .map(|(&k, &i)| transform(k).skip(i).next().unwrap())
    .collect();

  assert_eq!(encryption_keys[0], encryption_keys[1]);
  println!("the encryption key is {}", encryption_keys[0]);
}
