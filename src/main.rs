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

fn find_loop_size(public_key: Scalar) -> usize {
  for (i, key) in transform(7).enumerate() {
    if key == public_key {
      return i;
    }
  }
  unreachable!();
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
    .map(|&v| find_loop_size(v))
    .collect::<Vec<_>>();

  let encryption_keys = public_keys
    .iter()
    .rev()
    .zip(loop_sizes.iter())
    .map(|(&k, &i)| transform(k).skip(i).next().unwrap())
    .collect::<Vec<_>>();

  assert_eq!(encryption_keys[0], encryption_keys[1]);
  println!("the encryption key is {}", encryption_keys[0]);
}
