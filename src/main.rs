use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type Scalar = u64;

#[derive(Debug)]
struct Rule {
  ranges: Vec<(Scalar, Scalar)>,
}

impl Rule {
  fn check(&self, v: &Scalar) -> bool {
    self
      .ranges
      .iter()
      .map(|(min, max)| v >= min && v <= max)
      .fold(false, |a, b| a || b)
  }
}

fn parse_rule(input: &str) -> (&str, Rule) {
  let (name, ranges_str) = input.split(": ").next_tuple().unwrap();
  let ranges = ranges_str
    .split(" or ")
    .map(|range| {
      range
        .split('-')
        .map(|s| s.parse::<Scalar>().unwrap())
        .next_tuple::<(Scalar, Scalar)>()
        .unwrap()
    })
    .collect::<Vec<(Scalar, Scalar)>>();
  return (name, Rule { ranges: ranges });
}

fn parse_ticket(input: &str) -> Vec<Scalar> {
  return input
    .split(',')
    .map(|v| v.parse::<Scalar>().unwrap())
    .collect();
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let mut input = io::BufReader::new(File::open(&filename).unwrap())
    .lines()
    .filter_map(|s| s.ok());

  let mut rules: HashMap<String, Rule> = HashMap::new();

  while let Some(current) = input.next().filter(|s| s.len() > 0) {
    let (name, ranges) = parse_rule(&current);
    rules.insert(String::from(name), ranges);
  }

  input.next();
  let _own_ticket = parse_ticket(&input.next().unwrap());

  input.next();
  input.next();

  let result = input
    .map(|s| parse_ticket(&s))
    .map(|t| {
      t.iter()
        .filter(|v| {
          rules
            .values()
            .map(|r| !r.check(v))
            .fold(true, |a, b| a && b)
        })
        .fold(0, |a, b| a + b)
    })
    .fold(0, |a, b| a + b);

  println!("the result is {}", result);
}
