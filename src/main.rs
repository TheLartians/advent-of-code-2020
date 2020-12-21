
use std::env;
mod lib;
use lib::{parse_input, assemble_tiles, count_monsters};

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  count_monsters(&assemble_tiles(&parse_input(&filename)), true);
}
