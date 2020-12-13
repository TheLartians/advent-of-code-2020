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

  let start_time = input.next().unwrap().parse::<u64>().unwrap();
  let departure_times = input
    .next()
    .unwrap()
    .split(',')
    .filter(|&s| s != "x")
    .map(|s| s.parse::<u64>().unwrap())
    .collect::<Vec<u64>>();

  let mut current = start_time;
  'outer: loop {
    for bus in &departure_times {
      if current % bus == 0 {
        println!("the first bus is {} and arrives at {}", bus, current);
        println!("the result is {}", (current - start_time) * bus);
        break 'outer;
      }
    }
    current += 1;
  }
}
