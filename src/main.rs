use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;

type Scalar = i32;
type Direction = (Scalar, Scalar);

fn parse_direction<I: Iterator<Item = u8>>(input: &mut I) -> Option<Direction> {
  let c = input.next()?;
  return match c {
    b'e' => Some((2, 0)),
    b'w' => Some((-2, 0)),
    b's' => Some((
      match input.next().unwrap() {
        b'w' => -1,
        b'e' => 1,
        _ => unreachable!(),
      },
      -1,
    )),
    b'n' => Some((
      match input.next().unwrap() {
        b'w' => -1,
        b'e' => 1,
        _ => unreachable!(),
      },
      1,
    )),
    b'\n' => None,
    _ => unreachable!(),
  };
}

fn parse_instruction<I: Iterator<Item = u8>>(mut input: &mut I) -> Option<Vec<Direction>> {
  let mut result = Vec::new();
  while let Some(d) = parse_direction(&mut input) {
    result.push(d);
  }
  return if result.len() > 0 { Some(result) } else { None };
}

fn get_tile_position(instruction: &Vec<Direction>) -> (Scalar, Scalar) {
  let mut result = (0, 0);
  for i in instruction {
    result.0 += i.0;
    result.1 += i.1;
  }
  return result;
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();

  let file = File::open(filename).unwrap();
  let mut bytes = file.bytes().filter_map(|b| b.ok());

  let mut flipped = HashSet::new();

  while let Some(instruction) = parse_instruction(&mut bytes) {
    let tile_position = get_tile_position(&instruction);
    if flipped.contains(&tile_position) {
      flipped.remove(&tile_position);
    } else {
      flipped.insert(tile_position);
    }
  }

  println!("flipped tiles {:?}", flipped.len());
}
