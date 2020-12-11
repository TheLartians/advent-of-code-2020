use std::cmp;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

fn print_matrix<T: fmt::Display>(input: &Vec<T>, columns: usize, printer: fn(&T) -> char) {
  for i in 0..input.len() {
    print!("{}", printer(&input[i]));
    if (i + 1) % columns == 0 {
      print!("\n");
    }
  }
}

fn count_neighbors(input: &Vec<u8>, rows: usize, columns: usize) -> Vec<u8> {
  let mut result: Vec<u8> = vec![0; input.len()];

  let get_index = |i: usize, j: usize| {
    if i < columns && j < rows {
      return Ok(i + j * columns);
    } else {
      return Err(());
    }
  };

  let set_neighbouring = |res: &mut Vec<u8>, i, j| {
    if let Ok(idx) = get_index(i, j) {
      res[idx] += 1;
      return match input[idx] {
        b'#' | b'L' => true,
        _ => false,
      };
    } else {
      return true;
    }
  };

  let directions = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
  ];

  let get_directional_index = |i: usize, k: usize, d: i32| match d {
    1 => i + k,
    -1 => i.wrapping_sub(k),
    _ => i,
  };

  for i in 0..columns {
    for j in 0..rows {
      if input[get_index(i, j).unwrap()] == b'#' {
        let mut visible: Vec<bool> = directions.iter().map(|_| true).collect();
        for k in 1..cmp::max(rows, columns) {
          for (vi, (di, dj)) in directions.iter().enumerate() {
            if visible[vi] {
              visible[vi] &= !set_neighbouring(
                &mut result,
                get_directional_index(i, k, *di),
                get_directional_index(j, k, *dj),
              );
            }
          }
        }
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
  let rows = get_input().lines().count();
  let columns = get_input().lines().next().unwrap().unwrap().len();

  let mut state: Vec<u8> = get_input()
    .lines()
    .filter_map(|s| s.ok())
    .map(|s| s.bytes().collect::<Vec<u8>>())
    .flatten()
    .collect();

  // check if input is rectangular
  assert_eq!(state.len(), rows * columns);

  let mut step = 0;
  loop {
    let neighbors = count_neighbors(&state, rows, columns);
    let mut changed = false;
    for i in 0..state.len() {
      match state[i] {
        b'L' => {
          if neighbors[i] == 0 {
            state[i] = b'#';
            changed = true;
          }
        }
        b'#' => {
          if neighbors[i] >= 5 {
            state[i] = b'L';
            changed = true;
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

  print_matrix(&state, columns, |&v| v as char);
}
