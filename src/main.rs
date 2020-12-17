use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

type Scalar = usize;
type Ticket = Vec<Scalar>;

#[derive(Debug)]
struct Rule {
  ranges: Vec<(Scalar, Scalar)>,
}

type Rules = Vec<Rule>;

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

fn parse_ticket(input: &str) -> Ticket {
  return input
    .split(',')
    .map(|v| v.parse::<Scalar>().unwrap())
    .collect();
}

fn ticket_has_no_illegal_value(ticket: &Ticket, rules: &Rules) -> bool {
  ticket
    .iter()
    .map(|v| rules.iter().map(|r| r.check(v)).fold(false, |a, b| a || b))
    .fold(true, |a, b| a && b)
}

type FieldMatrix = Vec<Vec<bool>>;

fn find_rule_field_idx(fields: &Vec<bool>) -> Option<usize> {
  if fields.iter().fold(0, |a, &b| a + (b as usize)) == 1 {
    return Some(
      fields
        .iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .map(|(i, _)| i)
        .next()
        .unwrap(),
    );
  } else {
    return None;
  }
}

fn remove_value(ri: Scalar, fi: Scalar, valid: &mut FieldMatrix) {
  if valid[ri][fi] {
    valid[ri][fi] = false;
    if let Some(idx) = find_rule_field_idx(&valid[ri]) {
      for ri2 in (0..valid.len()).filter(|&i| i != ri) {
        remove_value(ri2, idx, valid);
      }
    }
  }
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let mut input = io::BufReader::new(File::open(&filename).unwrap())
    .lines()
    .filter_map(|s| s.ok());

  let mut rules: Rules = Rules::new();
  let mut rule_names: Vec<String> = Vec::new();

  while let Some(current) = input.next().filter(|s| s.len() > 0) {
    let (name, ranges) = parse_rule(&current);
    rules.push(ranges);
    rule_names.push(String::from(name));
  }

  input.next();
  let own_ticket = parse_ticket(&input.next().unwrap());

  input.next();
  input.next();

  let mut field_matrix: FieldMatrix = vec![vec![true; own_ticket.len()]; rules.len()];

  for ticket in input
    .map(|s| parse_ticket(&s))
    .filter(|t| ticket_has_no_illegal_value(&t, &rules))
  {
    for (fi, f) in ticket.iter().enumerate() {
      for (ri, _) in rules.iter().enumerate().filter(|(_, r)| !r.check(f)) {
        remove_value(ri, fi, &mut field_matrix);
      }
    }
  }

  let mut result = 1;
  for (ri, _) in rule_names
    .iter()
    .enumerate()
    .filter(|(_, s)| s.starts_with("departure"))
  {
    let idx = find_rule_field_idx(&field_matrix[ri]).unwrap();
    result *= own_ticket[idx];
  }

  println!("the result is {}", result);
}
