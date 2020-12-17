use itertools::iproduct;
use ndarray::Array4;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

fn parse_state(input: u8) -> Option<bool> {
  match input {
    b'#' => Some(true),
    b'.' => Some(false),
    _ => None,
  }
}

fn count_neighbors(state: &Array4<bool>) -> Array4<u8> {
  let mut result: Array4<u8> = Array4::zeros(state.raw_dim());
  let shape = result.raw_dim();

  let bounded_range =
    |d, i| (if i == 0 { 0 } else { -1 as isize })..(if i + 1 == shape[d] { 1 } else { 2 });

  for ((i, j, k, l), value) in result.indexed_iter_mut() {
    for (x, y, z, v) in iproduct!(
      bounded_range(0, i),
      bounded_range(1, j),
      bounded_range(2, k),
      bounded_range(3, l)
    ) {
      if state[[
        i.wrapping_add(x as usize),
        j.wrapping_add(y as usize),
        k.wrapping_add(z as usize),
        l.wrapping_add(v as usize),
      ]] {
        *value += 1;
      }
    }
    if state[[i, j, k, l]] {
      *value -= 1;
    }
  }

  return result;
}

fn update_state(state: &mut Array4<bool>) {
  let neighbors = count_neighbors(state);

  for (idx, value) in state.indexed_iter_mut() {
    if *value {
      match neighbors[idx] {
        2 | 3 => {}
        _ => *value = false,
      }
    } else {
      match neighbors[idx] {
        3 => *value = true,
        _ => {}
      }
    }
  }
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let input = io::BufReader::new(File::open(&filename).unwrap())
    .lines()
    .filter_map(|s| s.ok())
    .map(|s| {
      s.bytes()
        .map(|b| parse_state(b).unwrap())
        .collect::<Vec<bool>>()
    })
    .collect::<Vec<Vec<bool>>>();

  let iterations = args.next().unwrap().parse::<usize>().unwrap();

  let final_size = (
    1 + 2 * iterations,
    1 + 2 * iterations,
    1 + input.len() + 2 * iterations,
    1 + input[0].len() + 2 * iterations,
  );

  let mut state: Array4<bool> = Array4::from_elem(final_size, false);
  for i in 0..input.len() {
    for j in 0..input[0].len() {
      state[[
        final_size.0 / 2,
        final_size.1 / 2,
        final_size.2 / 2 - input.len() / 2 + i,
        final_size.3 / 2 - input[0].len() / 2 + j,
      ]] = input[i][j];
    }
  }

  let print_state = |i, state: &Array4<bool>| {
    println!(
      "cycle {}, {} active cubes",
      i,
      state.iter().filter(|&&v| v).count()
    );
    if iterations <= 2 {
      println!("{:?}\n", state.map(|&v| if v { '#' } else { '.' }));
    }
  };

  print_state(0, &state);
  for i in 0..iterations {
    update_state(&mut state);
    print_state(i + 1, &state)
  }
}
