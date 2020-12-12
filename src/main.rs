use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let get_input = || io::BufReader::new(File::open(&filename).unwrap());

  let trees = get_input()
    .lines()
    .filter_map(|s| s.ok())
    .map(|s| s.bytes().map(|b| b == b'#').collect::<Vec<bool>>());

  let mut position = 0;
  let mut collisions = 0;
  for row in trees {
    if row[position % row.len()] {
      collisions += 1;
    }
    position += 3;
  }
  println!("encountered {} trees", collisions);
}
