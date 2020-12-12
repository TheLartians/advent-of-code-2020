use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let get_input = || io::BufReader::new(File::open(&filename).unwrap());

  let mut input: Vec<i64> = get_input()
    .lines()
    .filter_map(|s| s.ok())
    .map(|s| s.parse::<i64>().unwrap())
    .collect();
  input.sort();

  let target = 2020;

  for v1 in &input {
    for v2 in &input {
      if input.binary_search(&(target - v1 - v2)).is_ok() {
        println!("the result is {}", v1 * v2 * (target - v1 - v2));
        return;
      }
    }
  }
}
