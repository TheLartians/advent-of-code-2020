use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let input = io::BufReader::new(File::open(filename).unwrap());

  let mut adaptors: Vec<u64> = input
    .lines()
    .filter_map(|s| s.ok())
    .filter(|s| s.len() > 0)
    .map(|s| s.parse::<u64>().unwrap())
    .collect();
  adaptors.sort();
  let target = adaptors.last().unwrap() + 3;

  let mut joltages: Vec<u64> = [0].to_vec();
  joltages.append(&mut adaptors);
  joltages.push(target);

  let differences = (0..joltages.len() - 1).map(|i| joltages[i + 1] - joltages[i]);
  let one_jolt_differences = differences.clone().filter(|v| v == &1).count();
  let three_jolt_differences = differences.clone().filter(|v| v == &3).count();

  println!(
    "there are {} one and {} three jolt differences",
    one_jolt_differences, three_jolt_differences
  );
  println!(
    "their product is {}",
    one_jolt_differences * three_jolt_differences
  );
}
