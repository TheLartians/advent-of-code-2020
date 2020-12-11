use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn _print_matrix<T: fmt::Display>(input: &Vec<T>, columns: usize, printer: fn(&T) -> char) {
  for i in 0..input.len() {
    print!("{}", printer(&input[i]));
    if (i + 1) % columns == 0 {
      print!("\n");
    }
  }
}

fn pad<T: Copy>(input: &mut Vec<T>, value: &T, count: usize) -> Vec<T> {
  let mut result = vec![*value; count];
  result.append(input);
  result.append(&mut vec![*value; count]);
  return result;
}

fn count_neighbors(input: &Vec<u8>, rows: usize, columns: usize) -> Vec<u8> {
  let mut result: Vec<u8> = vec![0; input.len()];
  let get_index = |i: usize, j: usize| i * columns + j;

  for i in 1..rows - 1 {
    for j in 1..columns - 1 {
      if input[get_index(i, j)] == b'#' {
        for ki in i - 1..i + 2 {
          for kj in j - 1..j + 2 {
            result[get_index(ki, kj)] += 1;
          }
        }
        result[get_index(i, j)] -= 1;
      }
    }
  }

  return result;
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let get_input = || io::BufReader::new(File::open(&filename).unwrap());
  let rows = get_input().lines().count() + 2;
  let columns = get_input().lines().next().unwrap().unwrap().len() + 2;

  let mut state: Vec<u8> = pad(
    &mut get_input()
      .lines()
      .filter_map(|s| s.ok())
      .map(|s| pad(&mut s.bytes().collect::<Vec<u8>>(), &b'.', 1))
      .flatten()
      .collect(),
    &b'.',
    columns,
  );

  assert_eq!(state.len(), rows * columns);

  let mut step = 0;
  loop {
    let neighbors = count_neighbors(&state, rows, columns);
    let mut changed = false;
    for i in 0..state.len() {
      match state[i] {
        b'L' => {
          if neighbors[i] == 0 {
            changed = true;
            state[i] = b'#'
          }
        }
        b'#' => {
          if neighbors[i] >= 4 {
            changed = true;
            state[i] = b'L'
          }
        }
        _ => {}
      }
    }
    if !changed {
      break;
    }
    step += 1;
    println!(
      "step {}: there are now {} seats occupied",
      step,
      state.iter().filter(|&&v| v == b'#').count()
    );
  }
}
