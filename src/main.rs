use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;
use std::cmp;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let input = io::BufReader::new(File::open(filename).unwrap());

  let mut adaptors: Vec<usize> = input
    .lines()
    .filter_map(|s| s.ok())
    .filter(|s| s.len() > 0)
    .map(|s| s.parse::<usize>().unwrap())
    .collect();
  adaptors.sort();
  let target = adaptors.last().unwrap() + 3;
  adaptors.push(target);

  let mut reachable_joltages: Vec<u64> = vec![0; target+1];
  reachable_joltages[0] = 1;

  for adaptor in &adaptors {
    for i in 1..cmp::min(3,*adaptor)+1 {
      reachable_joltages[*adaptor] += reachable_joltages[*adaptor-i];
    }
  }
  
  println!("there are {} possible configurations", reachable_joltages.last().unwrap());
}
