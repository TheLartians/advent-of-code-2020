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
    .map(|s| s.bytes().map(|b| b == b'#').collect::<Vec<bool>>())
    .collect::<Vec<Vec<bool>>>();

  let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  let result = slopes
    .iter()
    .map(|(right, down)| {
      let mut collisions = 0;
      let mut position = 0;
      for i in (0..trees.len()).step_by(*down) {
        let row = &trees[i];
        if row[position % row.len()] {
          collisions += 1;
        }
        position += right;
      }
      return collisions;
    })
    .fold(1 as i64, |a, b| a * b);

  println!("the result is {}", result);
}
