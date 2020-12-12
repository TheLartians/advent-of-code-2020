use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn is_valid_password(input: &str) -> bool {
  let mut it = input.split(' ');
  let policy = it
    .next()
    .unwrap()
    .split('-')
    .map(|s| s.parse::<usize>().unwrap());
  let letter = it.next().unwrap().bytes().next().unwrap();
  let pw = it.next().unwrap();

  return policy.fold(0, |c, i| {
    if pw.as_bytes()[i - 1] == letter {
      c + 1
    } else {
      c
    }
  }) == 1;
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
