use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

#[derive(Debug)]
enum State {
  ACTIVE,
  INACTIVE
}

fn parse_state(input: u8) -> Option<State> {
  match input {
    b'.' => Some(State::INACTIVE),
    b'#' => Some(State::ACTIVE),
    _ => None,
  }
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let input = io::BufReader::new(File::open(&filename).unwrap())
    .lines()
    .filter_map(|s| s.ok())
    .map(|s| s.bytes().map(|b| parse_state(b).unwrap()).collect::<Vec<State>>())
    .collect::<Vec<Vec<State>>>()
    ;
  println!("input {:?}", input);

}
