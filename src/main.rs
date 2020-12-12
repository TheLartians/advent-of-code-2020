use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;

#[derive(Debug)]
enum Instruction {
  N,
  S,
  E,
  W,
  L,
  R,
  F,
}

fn parse_direction(input: &str) -> (Instruction, i64) {
  let direction = match input.bytes().next().unwrap() {
    b'N' => Ok(Instruction::N),
    b'S' => Ok(Instruction::S),
    b'E' => Ok(Instruction::E),
    b'W' => Ok(Instruction::W),
    b'L' => Ok(Instruction::L),
    b'R' => Ok(Instruction::R),
    b'F' => Ok(Instruction::F),
    _ => Err(()),
  }
  .unwrap();
  let distance = input[1..].parse::<i64>().unwrap();
  return (direction, distance);
}

#[derive(Debug)]
struct Ferry {
  position: (i64, i64),
  waypoint: (i64, i64),
}

impl Ferry {
  fn move_wp_by_angle(&mut self, distance: i64, angle: f64) {
    self.waypoint.0 += (angle.cos() * (distance as f64)).round() as i64;
    self.waypoint.1 += (angle.sin() * (distance as f64)).round() as i64;
  }

  fn turn_wp_by_angle(&mut self, angle: f64) {
    let r = ((self.waypoint.0 * self.waypoint.0 + self.waypoint.1 * self.waypoint.1) as f64).sqrt();
    let angle = (self.waypoint.1 as f64).atan2(self.waypoint.0 as f64) + angle;
    self.waypoint = (
      (r * angle.cos()).round() as i64,
      (r * angle.sin()).round() as i64,
    );
  }

  fn move_forward(&mut self, distance: i64) {
    self.position = (
      self.position.0 + self.waypoint.0 * distance,
      self.position.1 + self.waypoint.1 * distance,
    );
  }
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename: String = args.next().unwrap();
  let get_input = || io::BufReader::new(File::open(&filename).unwrap());

  let input = get_input()
    .lines()
    .filter_map(|s| s.ok())
    .map(|s| parse_direction(&s));

  let mut ferry = Ferry {
    position: (0, 0),
    waypoint: (10, 1),
  };

  for (direction, distance) in input {
    match direction {
      Instruction::N => ferry.move_wp_by_angle(distance, PI / 2.),
      Instruction::S => ferry.move_wp_by_angle(distance, -PI / 2.),
      Instruction::E => ferry.move_wp_by_angle(distance, 0.),
      Instruction::W => ferry.move_wp_by_angle(distance, PI),
      Instruction::L => ferry.turn_wp_by_angle((distance as f64) / 180. * PI),
      Instruction::R => ferry.turn_wp_by_angle(-(distance as f64) / 180. * PI),
      Instruction::F => ferry.move_forward(distance),
    }

    println!(
      "{:?}{:<3}: ferry updated: {:?}. distance from start: {}",
      direction,
      distance,
      ferry,
      ferry.position.0.abs() + ferry.position.1.abs()
    )
  }
}
