use num::integer::lcm;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let get_input = || io::BufReader::new(File::open(&filename).unwrap());

  let mut input = get_input().lines().filter_map(|s| s.ok());

  type Scalar = usize;

  input.next();
  let departure_times = input
    .next()
    .unwrap()
    .split(',')
    .enumerate()
    .filter(|&(_, s)| s != "x")
    .map(|(i, s)| (i, s.parse::<Scalar>().unwrap()))
    .collect::<Vec<(usize, Scalar)>>();

  let mut t = 0;
  let mut period = 1;
  let mut current = 0;
  while current < departure_times.len() {
    let (i, b) = departure_times[current];
    if (t + i) % b == 0 {
      current += 1;
      period = lcm(period, b);
      println!("found {} at {} with period {}", current, t, period)
    }
    t += period;
  }
}
