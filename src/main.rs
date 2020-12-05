use std::env;
use std::fs;
use std::str::Bytes;

fn get_binary_partition_index(input: Bytes, sym: u8) -> u32 {
  let mut value = 0;
  let mut increment = (2 as u32).pow(input.len() as u32);
  for v in input {
    increment /= 2;
    if v == sym {
      value += increment
    }
  }
  return value;
}

fn get_seat_id(ticket: &str) -> u32 {
  let row = get_binary_partition_index((&ticket[..7]).bytes(), b'B');
  let column = get_binary_partition_index((&ticket[7..]).bytes(), b'R');
  return row * 8 + column;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1]).unwrap();

  let tickets: Vec<&str> = contents.split("\n").collect();
  let mut seat_indices: Vec<u32> = tickets
    .into_iter()
    .filter(|v| v.len() > 0)
    .map(|v| get_seat_id(v))
    .collect();

  seat_indices.sort();
  let mut current = seat_indices[0];
  for seat in seat_indices {
    if seat != current {
      println!("seat {} is unoccupied", current);
    } 
    current = seat + 1;
  }
}
