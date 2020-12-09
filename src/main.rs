use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let input = io::BufReader::new(File::open(filename).unwrap());
  let preamble_len = args.next().unwrap().parse::<usize>().unwrap();


  let numbers: Vec<i64> = input
    .lines()
    .filter_map(|s| s.ok())
    .filter(|s| s.len() > 0)
    .map(|s|s.parse::<i64>().unwrap())
    .collect();

  for i in preamble_len..numbers.len() {
    let mut is_sum = false;
    let preamble_indices = i-preamble_len..i;
    for j in preamble_indices.clone() {
      for k in preamble_indices.clone() {
        is_sum |= numbers[j] != numbers[k] && numbers[i] == numbers[j] + numbers[k];
      }
    }
    if !is_sum {
      println!("the number {} at {} is not a sum of previous {}", numbers[i], i, preamble_len);
    }
  }

}
