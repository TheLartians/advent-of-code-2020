use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::string::String;

fn find_invalid_number_index<T: Add<Output = T> + Eq + Copy>(
  numbers: &Vec<T>,
  preamble_len: usize,
) -> Result<usize, ()> {
  for i in preamble_len..numbers.len() {
    let mut is_sum = false;
    let preamble_indices = i - preamble_len..i;
    for j in preamble_indices.clone() {
      for k in preamble_indices.clone() {
        is_sum |= numbers[j] != numbers[k] && numbers[i] == numbers[j] + numbers[k];
      }
    }
    if !is_sum {
      return Ok(i);
    }
  }
  return Err(());
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let input = io::BufReader::new(File::open(filename).unwrap());
  let preamble_len = args.next().unwrap().parse::<usize>().unwrap();

  let numbers: Vec<u64> = input
    .lines()
    .filter_map(|s| s.ok())
    .filter(|s| s.len() > 0)
    .map(|s| s.parse::<u64>().unwrap())
    .collect();

  let invalid_index = find_invalid_number_index(&numbers, preamble_len).unwrap();
  println!(
    "the number {} at {} is not a sum of previous {}",
    numbers[invalid_index], invalid_index, preamble_len
  );

  let mut prefix_sum: Vec<u64> = vec![0; numbers.len() + 1];
  for i in 0..numbers.len() {
    prefix_sum[i + 1] = prefix_sum[i] + numbers[i];
  }

  for i in 1..numbers.len() {
    let search = numbers[invalid_index] + prefix_sum[i];
    if let Ok(j) = prefix_sum[i + 2..].binary_search(&search) {
      println!("we can get the number by summing {} to {}", i, i + j + 1);
      let min = numbers[i..i + j + 1].iter().min().unwrap();
      let max = numbers[i..i + j + 1].iter().max().unwrap();
      println!("The sum of the min and max values is {}", min + max);
    }
  }
}
