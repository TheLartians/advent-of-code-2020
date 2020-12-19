use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

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

fn matches_rule<'a>(id: &usize, rules: &HashMap<usize, Rule>, input: &'a str) -> Option<&'a str> {
  if input.len() == 0 {
    return None;
  }
  match rules.get(&id).unwrap() {
    Rule::Character { value } => {
      if input.as_bytes()[0] == *value {
        return Some(&input[1..]);
      } else {
        return None;
      }
    }
    Rule::Choices { choices } => {
      for sequence in choices {
        let mut current = input;
        let mut valid = true;
        for id2 in sequence {
          match matches_rule(id2, rules, current) {
            Some(v) => current = v,
            None => {
              valid = false;
              break;
            }
          }
        }
        if valid {
          return Some(current);
        }
      }
      return None;
    }
  }
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

  let result = input
    .filter(|s| s.len() > 0)
    .filter(|s| {
      if let Some(v) = matches_rule(&0, &rules, &s) {
        v.len() == 0
      } else {
        false
      }
    })
    .count();

  println!("result {}", result);
}
