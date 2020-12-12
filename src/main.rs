use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn is_valid_password(input: &str) -> bool {
  let mut it = input.split(' ');
  let policy: Vec<usize> = it
    .next()
    .unwrap()
    .split('-')
    .map(|s| s.parse::<usize>().unwrap())
    .collect();
  let letter = it.next().unwrap().bytes().next().unwrap();
  let pw = it.next().unwrap();
  let count = pw.bytes().filter(|&b| b == letter).count();
  return count >= policy[0] && count <= policy[1];
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let get_input = || io::BufReader::new(File::open(&filename).unwrap());

  let valid = get_input()
    .lines()
    .filter_map(|s| s.ok())
    .filter(|s| is_valid_password(s))
    .count();

  println!("found {} valid passwords", valid);
}
