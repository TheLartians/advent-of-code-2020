use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

#[macro_use]
extern crate generator;
use generator::{Generator, Gn, Scope};

#[derive(Debug)]
enum Rule {
  Character { value: u8 },
  Choices { choices: Vec<Vec<usize>> },
}

fn parse_rule(input: &str) -> (usize, Rule) {
  let (id, definition) = input.split(": ").next_tuple().unwrap();
  let rule = if definition.as_bytes()[0] == b'"' {
    Rule::Character {
      value: definition.as_bytes()[1],
    }
  } else {
    Rule::Choices {
      choices: definition
        .split(" | ")
        .map(|s| s.split(" ").map(|s| s.parse().unwrap()).collect())
        .collect(),
    }
  };
  return (id.parse().unwrap(), rule);
}

fn yield_all_sequence_matches<'a>(
  s: &mut Scope<(), &'a str>,
  sequence: &'a Vec<usize>,
  rules: &'a HashMap<usize, Rule>,
  input: &'a str,
  current: usize,
) {
  if sequence.len() > current {
    for next in for_all_matches(&sequence[current], rules, input) {
      yield_all_sequence_matches(s, sequence, rules, next, current + 1);
    }
  } else {
    s.yield_(input);
  }
}

fn for_all_matches<'a>(
  id: &'a usize,
  rules: &'a HashMap<usize, Rule>,
  input: &'a str,
) -> Generator<'a, (), &'a str> {
  return Gn::new_scoped(move |mut s| {
    if input.len() > 0 {
      match rules.get(&id).unwrap() {
        Rule::Character { value } => {
          if input.as_bytes()[0] == *value {
            s.yield_(&input[1..]);
          }
        }
        Rule::Choices { choices } => {
          for sequence in choices {
            yield_all_sequence_matches(&mut s, sequence, rules, input, 0);
          }
        }
      }
    }
    done!();
  });
}

fn matches_rule(id: &usize, rules: &HashMap<usize, Rule>, input: &str) -> bool {
  for v in for_all_matches(id, rules, input) {
    if v.len() == 0 {
      return true;
    }
  }
  return false;
}

fn main() {
  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let mut input = io::BufReader::new(File::open(&filename).unwrap())
    .lines()
    .filter_map(|line| line.ok());

  let mut rules: HashMap<usize, Rule> = HashMap::new();

  while let Some(line) = input.next().filter(|s| s.len() > 0) {
    let (id, rule) = parse_rule(&line);
    rules.insert(id, rule);
  }

  rules.insert(8, parse_rule("8: 42 | 42 8").1);
  rules.insert(11, parse_rule("11: 42 31 | 42 11 31").1);

  let result = input
    .filter(|s| s.len() > 0)
    .filter(|s| matches_rule(&0, &rules, &s))
    .count();

  println!("valid matches {}", result);
}
