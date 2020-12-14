use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

type Scalar = u64;

fn parse_mask(input: &str) -> (Scalar, Scalar) {
  let mut on_bits: Scalar = 0;
  let mut off_bits: Scalar = !0;
  input
    .bytes()
    .enumerate()
    .map(|(i, v)| (input.len() - i - 1, v))
    .for_each(|(i, v)| {
      if v == b'1' {
        on_bits |= 1 << i;
      } else if v == b'0' {
        off_bits &= !(1 << i);
      }
    });
  return (on_bits, off_bits);
}

fn parse_set(input: &str) -> (usize, Scalar) {
  lazy_static! {
    static ref SET_REGEX: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
  }
  let capture = SET_REGEX.captures(input).unwrap();
  let loc = capture[1].parse::<usize>().unwrap();
  let val = capture[2].parse::<Scalar>().unwrap();
  return (loc, val);
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let get_input = || io::BufReader::new(File::open(&filename).unwrap());

  let input = get_input().lines().filter_map(|s| s.ok());
  
  let (mut on_bits, mut off_bits) = (0, !0);
  let mut memory: HashMap<usize, Scalar> = HashMap::new();

  for line in input {
    if &line[..4] == "mask" {
      let new_mask = parse_mask(&line);
      on_bits = new_mask.0;
      off_bits = new_mask.1;
    } else {
      let (loc, val) = parse_set(&line);
      memory.insert(loc, (val | on_bits) & off_bits);
    }
  }

  println!(
    "sum of memory values: {:?}",
    memory.iter().map(|v| v.1).fold(0, |a, b| a + b)
  );
}
