use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::mem;
use std::string::String;

#[macro_use]
extern crate generator;
use generator::{Generator, Gn};

type Scalar = usize;

fn parse_mask(input: &str) -> (Scalar, Scalar) {
  let mut on_bits: Scalar = 0;
  let mut floating_bits: Scalar = 0;
  input
    .bytes()
    .enumerate()
    .map(|(i, v)| (input.len() - i - 1, v))
    .for_each(|(i, v)| {
      if v == b'1' {
        on_bits |= 1 << i;
      } else if v == b'X' {
        floating_bits |= 1 << i;
      }
    });
  return (on_bits, floating_bits);
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

fn for_all_floating<'a>(
  value: Scalar,
  floating_bits: &'a Scalar,
  current: usize,
) -> Generator<(), Scalar> {
  return Gn::new_scoped(move |mut s| {
    let n = mem::size_of_val(floating_bits) * 8;
    if current != n {
      for v in for_all_floating(value, floating_bits, current + 1) {
        if floating_bits & (1 << current) != 0 {
          let mask = 1 << current;
          s.yield_(v & !mask);
          s.yield_(v | mask);
        } else {
          s.yield_(v);
        }
      }
    } else {
      s.yield_(value);
    }
    done!();
  });
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let get_input = || io::BufReader::new(File::open(&filename).unwrap());

  let input = get_input().lines().filter_map(|s| s.ok());
  let (mut on_bits, mut floating_bits) = (0, 0);
  let mut memory: HashMap<usize, Scalar> = HashMap::new();

  for line in input {
    if &line[..4] == "mask" {
      let new_mask = parse_mask(&line);
      on_bits = new_mask.0;
      floating_bits = new_mask.1;
    } else {
      let (loc, val) = parse_set(&line);
      for floc in for_all_floating(loc | on_bits, &floating_bits, 0) {
        memory.insert(floc, val);
      }
    }
  }

  println!(
    "sum of memory values: {:?}",
    memory.iter().map(|v| v.1).fold(0, |a, b| a + b)
  );
}
