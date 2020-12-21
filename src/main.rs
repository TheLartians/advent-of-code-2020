use ndarray::prelude::s;
use ndarray::{Array, Array2, ArrayView2, Axis};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
extern crate num;

#[macro_use]
extern crate num_derive;

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

    pixels.extend(line.bytes().map(|b| match b {
      b'#' => true,
      b'.' => false,
      _ => unreachable!(),
    }));
    rows += 1;
  }

  let columns = pixels.len() / rows;
  let data: Array2<bool> = Array::from(pixels).into_shape([rows, columns]).unwrap();
  return Some(Tile { id: id, data: data });
}

fn as_char_array(data: &ArrayView2<bool>) -> Array2<char> {
  return data.map(|&v| if v { '#' } else { '.' });
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

#[derive(Debug, Copy, Clone, FromPrimitive, std::cmp::PartialEq)]
enum Side {
  TOP = 0,
  LEFT = 1,
  BOTTOM = 2,
  RIGHT = 3,
}

#[derive(Debug, Copy, Clone)]
struct TileOrientation {
  side: Side,
  flipped: bool,
}

fn get_all_tile_data_edges<'a>(data: &'a Array2<bool>) -> Vec<(u64, TileOrientation)> {
  let normal_edges = [
    (data.slice(s![0, ..]), (Side::TOP, false)),
    (
      data.slice(s![data.shape()[0] - 1, ..]),
      (Side::BOTTOM, true),
    ),
    (data.slice(s![.., 0]), (Side::LEFT, true)),
    (
      data.slice(s![.., data.shape()[1] - 1]),
      (Side::RIGHT, false),
    ),
  ];

  return normal_edges
    .iter()
    .map(|(e, (s, o))| {
      vec![
        (
          get_edge_hash(e),
          TileOrientation {
            side: *s,
            flipped: *o,
          },
        ),
        (
          get_edge_hash(e.into_iter().rev()),
          TileOrientation {
            side: *s,
            flipped: !o,
          },
        ),
      ]
    })
    .flatten()
    .collect();
}

#[derive(Debug, Clone)]
struct OrientedTile<'a> {
  data: ArrayView2<'a, bool>,
  edges: HashMap<usize, TileOrientation>,
  position: Option<(isize, isize)>,
}

impl TileOrientation {
  fn rotate_90(&mut self) {
    self.side = num::FromPrimitive::from_isize((self.side as isize + 1) % 4).unwrap();
  }

  fn flip(&mut self) {
    self.flipped = !self.flipped;
  }
}

impl OrientedTile<'_> {
  fn rotate_90(&mut self) {
    self.data.invert_axis(Axis(1));
    self.data.swap_axes(0, 1);
    for edge in &mut self.edges {
      edge.1.rotate_90();
    }
  }

  fn flip(&mut self) {
    self.data.invert_axis(Axis(0));
    for edge in &mut self.edges {
      match edge.1.side {
        Side::TOP | Side::BOTTOM => {
          edge.1.side = num::FromPrimitive::from_isize((edge.1.side as isize + 2) % 4).unwrap();
        }
        Side::LEFT | Side::RIGHT => {}
      };
      edge.1.flip();
    }
  }
}

fn assemble_puzzle(tile_idx: usize, tiles: &mut Vec<OrientedTile>) {
  // need to clone edges to avoid double borrow
  let tile_edges = tiles[tile_idx].edges.clone();
  let tile_position = tiles[tile_idx].position.unwrap().clone();

  for (edge_idx, tile_side) in tile_edges.iter() {
    {
      let edge = &mut tiles[*edge_idx];
      if edge.position.is_some() {
        continue;
      };

      while edge.edges.get(&tile_idx).unwrap().flipped == tile_side.flipped {
        edge.flip();
        edge.rotate_90();
      }

      while tile_side.side as isize != (edge.edges.get(&tile_idx).unwrap().side as isize + 2) % 4 {
        edge.rotate_90();
      }
      edge.position = match tile_side.side {
        Side::TOP => Some((tile_position.0, tile_position.1 - 1)),
        Side::RIGHT => Some((tile_position.0 + 1, tile_position.1)),
        Side::BOTTOM => Some((tile_position.0, tile_position.1 + 1)),
        Side::LEFT => Some((tile_position.0 - 1, tile_position.1)),
      };
    }
    assemble_puzzle(*edge_idx, tiles);
  }
}

fn count_shapes(kernel: &ArrayView2<bool>, data: &ArrayView2<bool>) -> (usize, Array2<bool>) {
  let mut result = 0;
  let mut monster_positions = Array2::from_elem(data.raw_dim(), false);
  for i in 0..data.shape()[0] - kernel.shape()[0] {
    for j in 0..data.shape()[1] - kernel.shape()[1] {
      let mut valid = true;
      for ((k, l), _) in kernel.indexed_iter() {
        valid &= (!*kernel.get([k, l]).unwrap()) || *data.get([i + k, j + l]).unwrap();
      }
      if valid {
        monster_positions
          .slice_mut(s![i..i + kernel.shape()[0], j..j + kernel.shape()[1]])
          .assign(&kernel.view());
        result += 1;
      }
    }
  }
  return (result, monster_positions);
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

  let mut edge_hashes: HashMap<u64, (usize, TileOrientation)> = HashMap::new();
  let mut adjacent_tiles: Vec<HashMap<usize, TileOrientation>> = vec![HashMap::new(); tiles.len()];

  for (idx, tile) in tiles.iter().enumerate() {
    for edge in get_all_tile_data_edges(&tile.data) {
      if let Some(partner) = edge_hashes.get(&edge.0) {
        adjacent_tiles[idx].insert(partner.0, edge.1);
        adjacent_tiles[partner.0].insert(idx, partner.1);
      } else {
        edge_hashes.insert(edge.0, (idx, edge.1));
      }
    }
  }

  let mut oriented_tiles: Vec<OrientedTile> = adjacent_tiles
    .iter()
    .enumerate()
    .map(|(i, edges)| OrientedTile {
      data: tiles[i].data.view(),
      edges: edges.iter().map(|(&k, &v)| (k, v)).collect(),
      position: None,
    })
    .collect();

  oriented_tiles[0].position = Some((0, 0));
  assemble_puzzle(0, &mut oriented_tiles);

  let min_position = oriented_tiles
    .iter()
    .filter_map(|tile| tile.position)
    .fold(oriented_tiles[0].position.unwrap(), |(pa, pb), (a, b)| {
      (pa.min(a), pb.min(b))
    });

  let boarder_thickness = 1;
  let tile_width = oriented_tiles[0].data.shape()[0] - 2 * boarder_thickness;
  let tile_height = tiles[0].data.shape()[1] - 2 * boarder_thickness;
  let tiles_per_side = (tiles.len() as f32).sqrt() as usize;
  let mut result_image: Array2<bool> = Array2::from_elem(
    [tile_height * tiles_per_side, tile_width * tiles_per_side],
    false,
  );

  for tile in oriented_tiles.iter().filter(|t| t.position.is_some()) {
    let x = (tile.position.unwrap().0 - min_position.0) as usize * tile_width;
    let y = (tile.position.unwrap().1 - min_position.1) as usize * tile_height;
    result_image
      .slice_mut(s![y..y + tile_height, x..x + tile_width])
      .assign(&tile.data.slice(s![
        boarder_thickness..tile_height + boarder_thickness,
        boarder_thickness..tile_width + boarder_thickness
      ]));
  }

  let monster: Array2<bool> = parse_tile(
    &mut format!(
      "Tile 0:{}",
      "
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "
        .replace(" ", ".")
    )
    .split('\n')
    .map(|s| String::from(s)),
  )
  .unwrap()
  .data;

  let mut result_tile = OrientedTile {
    data: result_image.view(),
    edges: HashMap::new(),
    position: None,
  };

  for i in 0..8 {
    let (monster_count, monsters) = count_shapes(&monster.view(), &result_tile.data);
    if monster_count > 0 {
      let mut visualization = as_char_array(&result_tile.data);
      monsters.indexed_iter().for_each(|((i, j), &v)| {
        if v {
          visualization[[i, j]] = 'O';
        }
      });

      println!(
        "reconstructed puzzle\n{}\n",
        format!("{:#}", visualization)
          .replace(",", "")
          .replace("[", "")
          .replace("]", "")
          .replace(" ", "")
      );

      println!("there are {} sea monsters", monster_count);
      println!(
        "{} pixels are not part of a sea monster",
        result_tile.data.iter().filter(|&&s| s).count()
          - monster.iter().filter(|&&s| s).count() * monster_count
      );
      break;
    }

    result_tile.rotate_90();
    if i % 4 == 0 {
      result_tile.flip();
    }
  }
}
