use ndarray::prelude::s;
use ndarray::{Array, Array2};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

#[derive(Debug)]
struct Tile {
  id: usize,
  data: Array2<bool>,
}

fn parse_tile<I: Iterator<Item = String>>(lines: &mut I) -> Option<Tile> {
  let first_line = lines.next().unwrap_or("".to_string());
  if first_line.len() == 0 {
    return None;
  }

  let id = first_line
    .split(" ")
    .skip(1)
    .map(|s| s[..s.len() - 1].parse::<usize>().unwrap())
    .next()
    .unwrap();
  let mut pixels: Vec<bool> = Vec::new();
  let mut rows = 0;

  while let Some(line) = lines.next() {
    if line.len() == 0 {
      break;
    }

    pixels.extend(line.bytes().map(|b| if b == b'#' { true } else { false }));
    rows += 1;
  }

  let columns = pixels.len() / rows;
  let data: Array2<bool> = Array::from(pixels).into_shape([rows, columns]).unwrap();
  return Some(Tile { id: id, data: data });
}

fn parse_tiles<I: Iterator<Item = String>>(mut it: &mut I) -> Vec<Tile> {
  let mut tiles = Vec::new();
  while let Some(tile) = parse_tile(&mut it) {
    tiles.push(tile);
  }
  return tiles;
}

fn get_edge_hash<'a, I: IntoIterator<Item = &'a bool>>(edge: I) -> u64 {
  let mut result: u64 = 0;
  for (i, v) in edge.into_iter().enumerate() {
    if *v {
      result |= 1 << i;
    }
  }
  return result;
}

fn get_all_tile_edges<'a>(data: &'a Array2<bool>) -> Vec<u64> {
  let normal_edges = [
    data.slice(s![0, ..]),
    data.slice(s![data.shape()[0] - 1, ..]),
    data.slice(s![.., 0]),
    data.slice(s![.., data.shape()[1] - 1]),
  ];

  return normal_edges
    .iter()
    .map(|e| vec![get_edge_hash(e), get_edge_hash(e.into_iter().rev())])
    .flatten()
    .collect::<Vec<u64>>();
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();

  let tiles = parse_tiles(
    &mut io::BufReader::new(File::open(&filename).unwrap())
      .lines()
      .filter_map(|line| line.ok()),
  );

  let mut edge_hashes: HashMap<u64, usize> = HashMap::new();
  let mut adjacent_tiles: Vec<HashSet<usize>> = vec![HashSet::new(); tiles.len()];

  for (idx, tile) in tiles.iter().enumerate() {
    for edge_hash in get_all_tile_edges(&tile.data) {
      if let Some(partner) = edge_hashes.get_mut(&edge_hash) {
        adjacent_tiles[idx].insert(*partner);
        adjacent_tiles[*partner].insert(idx);
      } else {
        edge_hashes.insert(edge_hash, idx);
      }
    }
  }

  let mut result = 1;
  for (idx, adjacent) in adjacent_tiles.iter().enumerate() {
    if adjacent.len() == 2 {
      result *= tiles[idx].id;
    }
  }

  println!("result: {}", result);
}
